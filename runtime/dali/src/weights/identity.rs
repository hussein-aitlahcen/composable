
//! Autogenerated weights for `identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	fn add_registrar(r: u32, ) -> Weight {
		(26_191_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((689_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(56_975_000 as Weight)
			// Standard Error: 117_000
			.saturating_add((687_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 19_000
			.saturating_add((967_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	fn set_subs_new(s: u32, ) -> Weight {
		(49_208_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((6_994_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	fn set_subs_old(p: u32, ) -> Weight {
		(50_059_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((2_147_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:32)
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(59_653_000 as Weight)
			// Standard Error: 68_000
			.saturating_add((487_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 10_000
			.saturating_add((2_184_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 10_000
			.saturating_add((529_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(64_040_000 as Weight)
			// Standard Error: 52_000
			.saturating_add((637_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 8_000
			.saturating_add((926_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(61_061_000 as Weight)
			// Standard Error: 57_000
			.saturating_add((70_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 9_000
			.saturating_add((956_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fee(r: u32, ) -> Weight {
		(10_087_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((509_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_account_id(r: u32, ) -> Weight {
		(10_183_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((496_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fields(r: u32, ) -> Weight {
		(10_099_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((481_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(41_970_000 as Weight)
			// Standard Error: 50_000
			.saturating_add((687_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 7_000
			.saturating_add((922_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Identity SuperOf (r:0 w:32)
	fn kill_identity(r: u32, s: u32, _x: u32, ) -> Weight {
		(89_613_000 as Weight)
			// Standard Error: 89_000
			.saturating_add((339_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 13_000
			.saturating_add((2_158_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn add_sub(s: u32, ) -> Weight {
		(66_587_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((344_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	fn rename_sub(s: u32, ) -> Weight {
		(20_258_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((161_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn remove_sub(s: u32, ) -> Weight {
		(67_207_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((404_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn quit_sub(s: u32, ) -> Weight {
		(43_057_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((447_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
