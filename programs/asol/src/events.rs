//! Crate events

use anchor_lang::prelude::*;

use crate::*;

/// Emitted when an [Aggregate] is created.
#[event]
pub struct NewAggregateEvent {
    /// Aggregate
    #[index]
    pub aggregate: Pubkey,
    /// Curator.
    pub curator: Pubkey,
    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when a [StakePool] is added.
#[event]
pub struct AddStakePoolEvent {
    /// Aggregate
    #[index]
    pub aggregate: Pubkey,
    /// Stake pool
    #[index]
    pub stake_pool: Pubkey,

    /// The [Aggregate::curator].
    pub curator: Pubkey,
    /// The [Mint].
    pub mint: Pubkey,
    /// The accounting method used.
    pub accounting_method: AccountingMethod,

    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when an [Aggregate]'s curator is modified.
#[event]
pub struct SetCuratorEvent {
    /// Aggregate
    #[index]
    pub aggregate: Pubkey,

    /// The new [Aggregate::curator].
    pub curator: Pubkey,
    /// The previous [Aggregate::curator].
    pub previous_curator: Pubkey,
    /// The [Aggregate::curator_setter].
    pub curator_setter: Pubkey,

    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when ASol is minted.
#[event]
pub struct MintASolEvent {
    /// Depositor
    #[index]
    pub depositor: Pubkey,

    /// The mint of the stake pool token deposited.
    #[index]
    pub stake_pool_mint: Pubkey,

    /// Accounting method used.
    #[index]
    pub accounting_method: AccountingMethod,

    /// Amount of stake pool tokens deposited.
    pub deposit_amount: u64,

    /// Amount of aSOL minted.
    pub mint_amount: u64,

    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Information about an aggregate.
#[event]
pub struct AggregateInfoEvent {
    /// Pool snapshot.
    pub snapshot: Snapshot,
    /// Time that the info was fetched.
    pub timestamp: i64,
}
