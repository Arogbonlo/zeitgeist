use sp_runtime::Perbill;

// Distribution

/// Total ZTG amount for airdrops
pub const AIRDROPS: u128 = 3_000_000;

/// Total ZTG amount for collators
pub const COLLATORS: u128 = 10_000_000;

/// Total ZTG amount for community incentives
pub const COMMUNITY_INCENTIVES: u128 = 2_000_000;

/// Total ZTG amount for liquidity mining
pub const LIQUIDITY_MINING: u128 = 10_000_000;

/// Total ZTG amount for parachain lease
pub const PARACHAIN_LEASE: u128 = 40_000_000;

/// Total ZTG amount for public sale
pub const PUBLIC_SALE: u128 = 10_000_000;

/// Total ZTG amount for seed sale
pub const SEED_SALE: u128 = 7_000_000;

/// Total ZTG amount for strategic sale
pub const STRATEGIC_SALE: u128 = 8_000_000;

/// Total ZTG amount for Team and advisors
pub const TEAM_AND_ADVISORS: u128 = 15_000_000;

/// Total ZTG amount for Zeitgesit foundation
pub const ZEITGEIST_FOUNDATION: u128 = 15_000_000;

// Inflation

/// Perthousand crowdloan inflation. 1.5%
pub const CROWDLOAN_PTD: Perbill = Perbill::from_perthousand(15);

/// Perthousand liquidity mining inflation. 2%
pub const LIQUIDITY_MINING_PTD: Perbill = Perbill::from_perthousand(20);

/// Perthousand collator staking inflation. 1.5%
pub const STAKING_PTD: Perbill = Perbill::from_perthousand(15);
