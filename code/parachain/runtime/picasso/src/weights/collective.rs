
//! Autogenerated weights for `collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `dev`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/gpsh9wvfcrwyck2nw62gpkqhf0bhc0cw-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=2
// --repeat=2
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> collective::WeightInfo for WeightInfo<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 19_958 nanoseconds.
		Weight::from_ref_time(19_958_000 as u64)
			// Standard Error: 1_851_139
			.saturating_add(Weight::from_ref_time(7_135_927 as u64).saturating_mul(m as u64))
			// Standard Error: 1_851_139
			.saturating_add(Weight::from_ref_time(7_055_712 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 28_833 nanoseconds.
		Weight::from_ref_time(28_083_703 as u64)
			// Standard Error: 6_577
			.saturating_add(Weight::from_ref_time(3_584 as u64).saturating_mul(b as u64))
			// Standard Error: 67_970
			.saturating_add(Weight::from_ref_time(58_712 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(_b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 30_126 nanoseconds.
		Weight::from_ref_time(39_528_217 as u64)
			// Standard Error: 105_378
			.saturating_add(Weight::from_ref_time(15_565 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(_b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 33_959 nanoseconds.
		Weight::from_ref_time(63_701_976 as u64)
			// Standard Error: 254_813
			.saturating_add(Weight::from_ref_time(44_428 as u64).saturating_mul(m as u64))
			// Standard Error: 252_240
			.saturating_add(Weight::from_ref_time(408_883 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(_m: u32, ) -> Weight {
		// Minimum execution time: 37_917 nanoseconds.
		Weight::from_ref_time(52_984_078 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(_m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 41_042 nanoseconds.
		Weight::from_ref_time(47_667_980 as u64)
			// Standard Error: 94_968
			.saturating_add(Weight::from_ref_time(395_207 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 63_084 nanoseconds.
		Weight::from_ref_time(55_992_927 as u64)
			// Standard Error: 10_354
			.saturating_add(Weight::from_ref_time(7_911 as u64).saturating_mul(b as u64))
			// Standard Error: 110_335
			.saturating_add(Weight::from_ref_time(39_388 as u64).saturating_mul(m as u64))
			// Standard Error: 106_991
			.saturating_add(Weight::from_ref_time(300_608 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 40_208 nanoseconds.
		Weight::from_ref_time(23_193_092 as u64)
			// Standard Error: 104_705
			.saturating_add(Weight::from_ref_time(225_692 as u64).saturating_mul(m as u64))
			// Standard Error: 101_533
			.saturating_add(Weight::from_ref_time(446_136 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 57_958 nanoseconds.
		Weight::from_ref_time(26_639_111 as u64)
			// Standard Error: 17_710
			.saturating_add(Weight::from_ref_time(23_929 as u64).saturating_mul(b as u64))
			// Standard Error: 188_728
			.saturating_add(Weight::from_ref_time(107_208 as u64).saturating_mul(m as u64))
			// Standard Error: 183_009
			.saturating_add(Weight::from_ref_time(532_626 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 22_875 nanoseconds.
		Weight::from_ref_time(26_335_176 as u64)
			// Standard Error: 73_769
			.saturating_add(Weight::from_ref_time(248_323 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
