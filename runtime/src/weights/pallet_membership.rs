//! Autogenerated weights for pallet_membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:// ./target/release/zeitgeist// benchmark// --chain=dev// --steps=50// --repeat=20// --pallet=pallet_membership// --extrinsic=*// --execution=wasm// --wasm-execution=compiled// --heap-pages=4096// --template=./misc/frame_weight_template.hbs// --output=./runtime/src/weights/
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions for pallet_membership (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::weights::WeightInfo for WeightInfo<T> {
		// Storage: AdvisoryCommitteeMembership Members (r:1 w:1)
		// Storage: AdvisoryCommittee Proposals (r:1 w:0)
		// Storage: AdvisoryCommittee Members (r:0 w:1)
		// Storage: AdvisoryCommittee Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		(31_674_000 as Weight)		
		// Standard Error: 12_000

			.saturating_add((258_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
		// Storage: AdvisoryCommitteeMembership Members (r:1 w:1)
		// Storage: AdvisoryCommittee Proposals (r:1 w:0)
		// Storage: AdvisoryCommitteeMembership Prime (r:1 w:0)
		// Storage: AdvisoryCommittee Members (r:0 w:1)
		// Storage: AdvisoryCommittee Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		(42_912_000 as Weight)		
		// Standard Error: 5_000

			.saturating_add((149_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
		// Storage: AdvisoryCommitteeMembership Members (r:1 w:1)
		// Storage: AdvisoryCommittee Proposals (r:1 w:0)
		// Storage: AdvisoryCommitteeMembership Prime (r:1 w:0)
		// Storage: AdvisoryCommittee Members (r:0 w:1)
		// Storage: AdvisoryCommittee Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		(40_127_000 as Weight)		
		// Standard Error: 5_000

			.saturating_add((196_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
		// Storage: AdvisoryCommitteeMembership Members (r:1 w:1)
		// Storage: AdvisoryCommittee Proposals (r:1 w:0)
		// Storage: AdvisoryCommitteeMembership Prime (r:1 w:0)
		// Storage: AdvisoryCommittee Members (r:0 w:1)
		// Storage: AdvisoryCommittee Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		(39_227_000 as Weight)		
		// Standard Error: 5_000

			.saturating_add((415_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
		// Storage: AdvisoryCommitteeMembership Members (r:1 w:1)
		// Storage: AdvisoryCommittee Proposals (r:1 w:0)
		// Storage: AdvisoryCommitteeMembership Prime (r:1 w:1)
		// Storage: AdvisoryCommittee Members (r:0 w:1)
		// Storage: AdvisoryCommittee Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		(43_082_000 as Weight)		
		// Standard Error: 5_000

			.saturating_add((150_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
		// Storage: AdvisoryCommitteeMembership Members (r:1 w:0)
		// Storage: AdvisoryCommitteeMembership Prime (r:0 w:1)
		// Storage: AdvisoryCommittee Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		(11_096_000 as Weight)		
		// Standard Error: 2_000

			.saturating_add((145_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
		// Storage: AdvisoryCommitteeMembership Prime (r:0 w:1)
		// Storage: AdvisoryCommittee Prime (r:0 w:1)
	fn clear_prime(_m: u32, ) -> Weight {
		(4_063_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
