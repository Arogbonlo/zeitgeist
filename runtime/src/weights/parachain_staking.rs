//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=parachain_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions for parachain_staking (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> parachain_staking::weights::WeightInfo for WeightInfo<T> {
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_staking_expectations() -> Weight {
		(47_480_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_inflation() -> Weight {
		(137_950_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_account() -> Weight {
		(45_970_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_reserve_percent() -> Weight {
		(44_151_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	fn set_total_selected() -> Weight {
		(43_251_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		(42_620_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		(142_330_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: ParachainStaking NominatorState2 (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn join_candidates(x: u32, ) -> Weight {
		(161_207_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((250_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: ParachainStaking ExitQueue2 (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn leave_candidates(x: u32, ) -> Weight {
		(150_919_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((231_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn go_offline() -> Weight {
		(77_761_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn go_online() -> Weight {
		(78_871_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn candidate_bond_more() -> Weight {
		(125_981_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn candidate_bond_less() -> Weight {
		(126_171_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking NominatorState2 (r:1 w:1)
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn nominate(x: u32, y: u32, ) -> Weight {
		(129_946_000 as Weight)
			// Standard Error: 79_000
			.saturating_add((1_048_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 29_000
			.saturating_add((1_039_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking NominatorState2 (r:1 w:1)
	// Storage: ParachainStaking ExitQueue2 (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn leave_nominators(x: u32, ) -> Weight {
		(73_691_000 as Weight)
			// Standard Error: 64_000
			.saturating_add((188_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking NominatorState2 (r:1 w:1)
	// Storage: ParachainStaking ExitQueue2 (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn revoke_nomination() -> Weight {
		(68_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking NominatorState2 (r:1 w:1)
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn nominator_bond_more() -> Weight {
		(130_991_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking NominatorState2 (r:1 w:1)
	// Storage: ParachainStaking CollatorState2 (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn nominator_bond_less() -> Weight {
		(130_601_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking Points (r:1 w:1)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: System Account (r:34 w:33)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:3)
	// Storage: ParachainStaking ExitQueue2 (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CollatorState2 (r:2 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	fn active_on_initialize(x: u32, y: u32, ) -> Weight {
		(2_378_867_000 as Weight)
			// Standard Error: 5_219_000
			.saturating_add((88_921_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 138_000
			.saturating_add((3_519_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(52 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(46 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn passive_on_initialize() -> Weight {
		(10_960_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}
