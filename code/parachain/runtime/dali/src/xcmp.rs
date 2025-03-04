use super::*;
use common::{
	fees::{PriceConverter, WeightToFeeConverter},
	governance::native::{EnsureRootOrHalfNativeTechnical, NativeCouncilCollective},
	xcmp::*,
};
use composable_traits::xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use frame_support::{
	log, parameter_types,
	traits::{Everything, OriginTrait, PalletInfoAccess},
};
use orml_traits::{
	location::{AbsoluteReserveProvider, RelativeReserveProvider, Reserve},
	parameter_type_with_key,
};
use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::Convert;
use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, BackingToPlurality, EnsureXcmOrigin,
	FixedWeightBounds, LocationInverter, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeRevenue, TakeWeightCredit,
};
use xcm_executor::{
	traits::{DropAssets, FilterAssetLocation},
	Assets, XcmExecutor,
};

parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	pub AssetsPalletLocation: MultiLocation =
		PalletInstance(<super::Assets as PalletInfoAccess>::index() as u8).into();
}

pub type Barrier = (
	AllowUnpaidExecutionFrom<ThisChain<ParachainInfo>>,
	AllowKnownQueryResponses<RelayerXcm>,
	AllowSubscriptionsFrom<ParentOrSiblings>,
	AllowTopLevelPaidExecutionFrom<Everything>,
	TakeWeightCredit,
);

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
	HereToTreasury,
);

pub struct HereToTreasury;

impl xcm_executor::traits::Convert<MultiLocation, AccountId> for HereToTreasury {
	fn convert_ref(value: impl core::borrow::Borrow<MultiLocation>) -> Result<AccountId, ()> {
		if value.borrow() == &(MultiLocation { parents: 0, interior: Here }) {
			Ok(TreasuryAccount::get())
		} else {
			Err(())
		}
	}
}

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

pub struct StaticAssetsMap;
impl XcmpAssets for StaticAssetsMap {}

pub type LocalAssetTransactor = MultiCurrencyAdapter<
	crate::Assets,
	UnknownTokens,
	IsNativeConcrete<CurrencyId, AssetsIdConverter>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	AssetsIdConverter,
	DepositToAlternative<TreasuryAccount, Tokens, CurrencyId, AccountId, Balance>,
>;

pub struct RelayReserveFromParachain;
impl FilterAssetLocation for RelayReserveFromParachain {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		// NOTE: In Acala there is not such thing
		// if asset is KSM and send from some parachain then allow for  that
		AbsoluteReserveProvider::reserve(asset) == Some(MultiLocation::parent()) &&
			matches!(origin, MultiLocation { parents: 1, interior: X1(Parachain(_)) })
	}
}

type IsReserveAssetLocationFilter =
	(MultiNativeAsset<AbsoluteReserveProvider>, RelayReserveFromParachain);

type AssetsIdConverter =
	CurrencyIdConvert<AssetsRegistry, CurrencyId, ParachainInfo, StaticAssetsMap>;

pub type Trader = TransactionFeePoolTrader<
	AssetsIdConverter,
	PriceConverter<AssetsRegistry>,
	ToTreasury<AssetsIdConverter, crate::Assets, TreasuryAccount>,
	WeightToFeeConverter,
>;

pub struct CaptureDropAssets<
	Treasury: TakeRevenue,
	PriceConverter,
	AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
>(PhantomData<(Treasury, PriceConverter, AssetConverter)>);

/// if asset  put  into Holding Registry of XCM VM, but did nothing to this
/// or if  too small to pay weight,
/// it will get here
/// if asset location and origin is known, put into treasury,
/// else if asset location and origin not know, hash it until it will be added
impl<
		Treasury: TakeRevenue,
		PriceConverter,
		AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
	> DropAssets for CaptureDropAssets<Treasury, PriceConverter, AssetConverter>
{
	fn drop_assets(origin: &MultiLocation, assets: Assets) -> u64 {
		let multi_assets: Vec<MultiAsset> = assets.into();
		let mut can_return_on_request = vec![];
		log::info!(target : "xcmp", "drop_assets");
		let mut weight = 0;
		for asset in multi_assets {
			if let MultiAsset { id: Concrete(location), fun: Fungible(_amount) } = asset.clone() {
				if let Some(_converted) = AssetConverter::convert(location) {
					Treasury::take_revenue(asset);
				} else {
					can_return_on_request.push(asset);
				}
			} else {
				can_return_on_request.push(asset);
			}
		}
		if !can_return_on_request.is_empty() {
			weight += RelayerXcm::drop_assets(origin, can_return_on_request.into());
		}
		weight
	}
}

pub type CaptureAssetTrap = CaptureDropAssets<
	ToTreasury<AssetsIdConverter, crate::Assets, TreasuryAccount>,
	PriceConverter<AssetsRegistry>,
	AssetsIdConverter,
>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = IsReserveAssetLocationFilter;
	type IsTeleporter = (); // <- should be enough to allow teleportation of PICA
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;

	type Trader = Trader;
	type ResponseHandler = RelayerXcm;

	type SubscriptionService = RelayerXcm;
	type AssetClaims = RelayerXcm;
	type AssetTrap = CaptureAssetTrap;
}

parameter_type_with_key! {
	// 1. use configured pessimistic asset min fee for target chain / asset pair
	// 2. use built int
	// 3. allow to transfer anyway (let not lock assets on our chain for now)
	// until XCM v4
	pub ParachainMinFee: |location: MultiLocation| -> Option<Balance> {
		#[allow(clippy::match_ref_pats)] // false positive
		#[allow(clippy::match_single_binding)]
		let parents = location.parents;
		let interior = location.first_interior();

		let location = XcmAssetLocation::new(location.clone());
		if let Some(Parachain(id)) = interior {
			if let Some(amount) = AssetsRegistry::min_xcm_fee(ParaId::from(*id), location) {
				return Some(amount)
			}
		}

		match (parents, interior) {
			(1, None) => Some(400_000),
			_ => None,
		}
	};
}

impl orml_xtokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = AssetsIdConverter;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = ThisLocal;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = XcmMaxAssetsForTransfer;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type ReserveProvider = RelativeReserveProvider;
}

impl orml_unknown_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	// One XCM operation is 200_000_000 weight, cross-chain transfer ~= 2x of transfer.
	pub const UnitWeightCost: u64 = 200_000_000;
	pub const MaxInstructions: u32 = 100;
}

pub fn xcm_asset_fee_estimator(instructions: u8, asset_id: CurrencyId) -> Balance {
	assert!((instructions as u32) <= MaxInstructions::get());
	let total_weight = UnitWeightCost::get() * instructions as u64;
	Trader::weight_to_asset(total_weight, asset_id)
		.expect("use only in simulator")
		.1
}

pub fn xcm_fee_estimator(instructions: u8) -> u64 {
	assert!((instructions as u32) <= MaxInstructions::get());
	UnitWeightCost::get() * instructions as u64
}

parameter_types! {
	pub const CollectiveBodyId: BodyId = BodyId::Unit;
}

parameter_types! {
	pub const CouncilBodyId: BodyId = BodyId::Executive;
}

pub type CouncilToPlurality = BackingToPlurality<
	RuntimeOrigin,
	collective::Origin<Runtime, NativeCouncilCollective>,
	CouncilBodyId,
>;

pub struct RootToHereLocation<RuntimeOrigin, AccountId, Network>(
	PhantomData<(RuntimeOrigin, AccountId, Network)>,
);
impl<Origin: OriginTrait + Clone, AccountId: Into<[u8; 32]>, Network: Get<NetworkId>>
	xcm_executor::traits::Convert<Origin, MultiLocation>
	for RootToHereLocation<Origin, AccountId, Network>
where
	Origin::PalletsOrigin: From<system::RawOrigin<AccountId>>
		+ TryInto<system::RawOrigin<AccountId>, Error = Origin::PalletsOrigin>,
{
	fn convert(o: Origin) -> Result<MultiLocation, Origin> {
		o.try_with_caller(|caller| match caller.try_into() {
			Ok(system::RawOrigin::Root) => Ok(Here.into()),
			Ok(other) => Err(other.into()),
			Err(other) => Err(other),
		})
	}
}

pub type RootOrNativeCouncilOrSignedLocation = (RootOrNativeCouncilLocation, LocalOriginToLocation);

pub type RootOrNativeCouncilLocation =
	(CouncilToPlurality, RootToHereLocation<RuntimeOrigin, AccountId, RelayNetwork>);

pub struct RootExecuteFilter;

impl Contains<(MultiLocation, Xcm<RuntimeCall>)> for RootExecuteFilter {
	fn contains((origin, _call): &(MultiLocation, Xcm<RuntimeCall>)) -> bool {
		origin == &MultiLocation { parents: 0, interior: Here }
	}
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmRouter = XcmRouter;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, RootOrNativeCouncilLocation>;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, RootOrNativeCouncilOrSignedLocation>;
	type XcmExecuteFilter = RootExecuteFilter;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type LocationInverter = LocationInverter<Ancestry>;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = VERSION_DISCOVERY_QUEUE_SIZE;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = RelayerXcm;
	type ChannelInfo = ParachainSystem;
	type ExecuteOverweightOrigin = EnsureRootOrHalfNativeCouncil;
	type ControllerOrigin = EnsureRootOrHalfNativeTechnical;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRootOrHalfNativeCouncil;
}
