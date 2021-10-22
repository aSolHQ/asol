use anchor_lang::prelude::*;

use crate::{Snapshot, StakePoolSnapshot, ASOL, SOL};

/// Contains the info of the aggregate token.
/// Make sure to allocate enough storage to handle a lot of stake pools.
#[account]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Aggregate {
    /// The [crate_token::CrateToken].
    pub crate_token: Pubkey,
    /// Bump.
    pub bump: u8,

    /// Account that can add or remove stake pools from the aggregate.
    pub curator: Pubkey,
    /// Account that can change who the curator is.
    pub curator_setter: Pubkey,

    /// A stake pool.
    pub stake_pools: Vec<StakePoolMeta>,

    /// Latest snapshot of the aggregate.
    pub latest_snapshot: Snapshot,
    /// When the latest snapshot was taken.
    pub latest_snapshot_ts: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct StakePoolMeta {
    /// Mint of the stake pool.
    pub mint: Pubkey,
    /// The accounting method.
    pub accounting_method: AccountingMethod,
}

/// Contains the state of the [StakePoolMeta].
/// Currently this is just used for TVL tracking.
#[account]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct StakePool {
    /// The [Aggregate].
    pub aggregate: Pubkey,
    /// Mint of the stake pool.
    pub mint: Pubkey,
    /// The bump.
    pub bump: u8,

    /// Accounting method the stake pool uses.
    pub accounting_method: AccountingMethod,

    /// Statistics on the stake pool.
    pub stats: StakePoolStats,

    /// The latest snapshot of the [StakePool].
    pub latest_snapshot: StakePoolStateSnapshot,
}

/// A balance snapshot of a stake pool.
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct StakePoolStats {
    /// Total amount of [StakePool::mint] tokens ever deposited.
    pub total_amount_deposited: u64,
    /// Total amount of [Aggregate::crate_token] tokens ever minted from this pool.
    pub total_amount_minted: ASOL,
}

/// A balance snapshot of a stake pool.
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct StakePoolStateSnapshot {
    /// Aggregate [SOL] balance.
    pub aggregate_balance_sol: SOL,
    /// Aggregate [ASOL] total supply.
    pub aggregate_supply: ASOL,
    /// Stake pool snapshot information.
    pub snapshot: StakePoolSnapshot,
    /// Time the last snapshot was taken.
    pub snapshot_ts: i64,
}

/// The accounting method of the stake pool.
#[repr(C)]
#[derive(
    AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum AccountingMethod {
    /// Marinade mSOL.
    Marinade,
    /// Lido stSOL.
    Lido,
}

impl Default for AccountingMethod {
    fn default() -> Self {
        AccountingMethod::Marinade
    }
}
