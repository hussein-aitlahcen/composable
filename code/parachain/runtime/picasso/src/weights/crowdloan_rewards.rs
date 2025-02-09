//! Autogenerated weights for `crowdloan_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-17, STEPS: `10`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=crowdloan-rewards
// --extrinsic=*
// --steps=10
// --repeat=5
// --raw
// --output=runtime/picasso/src/weights/crowdloan_rewards.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `crowdloan_rewards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crowdloan_rewards::weights::WeightInfo for WeightInfo<T> {
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:0)
	// Storage: CrowdloanRewards Rewards (r:1001 w:1000)
	// Storage: CrowdloanRewards TotalContributors (r:0 w:1)
	// Storage: CrowdloanRewards TotalRewards (r:0 w:1)
	fn populate(x: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 109_000
			.saturating_add(Weight::from_ref_time(6_792_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x as u64)))
	}
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:1)
	fn initialize(x: u32, ) -> Weight {
		Weight::from_ref_time(33_355_000_u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:0)
	// Storage: CrowdloanRewards Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CrowdloanRewards ClaimedRewards (r:1 w:1)
	// Storage: CrowdloanRewards Associations (r:0 w:1)
	fn associate(x: u32, ) -> Weight {
		Weight::from_ref_time(169_323_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(8_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: CrowdloanRewards Associations (r:1 w:0)
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:0)
	// Storage: CrowdloanRewards Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CrowdloanRewards ClaimedRewards (r:1 w:1)
	fn claim(x: u32, ) -> Weight {
		Weight::from_ref_time(94_034_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(31_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn unlock_rewards_for(_x: u32) -> Weight {
    Weight::from_ref_time(10_000_u64)
	}
}
