//! aSOL: an aggregated Solana stake pool.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

mod account_validators;
mod pool;

pub mod accounting;
pub mod events;
pub mod snapshot;
pub mod state;
pub mod types;

use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use anchor_spl::token::{Mint, Token, TokenAccount};
use vipers::validate::Validate;

pub use events::*;
pub use snapshot::*;
pub use state::*;
pub use types::*;

/// Maximum number of stake pools supported.
pub const MAX_STAKE_POOLS: usize = 30;

/// Number of decimals in lamports.
pub const LAMPORTS_DECIMALS: u8 = 9;

/// The minimum amount of liquidity in the pool for the "exact calculation" of SOL/ASOL price to be used.
/// If the amount of SOL in the pool is less than this number, the price of ASOL is pegged to 1 SOL.
pub const MIN_LIQUIDITY_FOR_EXACT_CALCULATION: u64 = LAMPORTS_PER_SOL;

declare_id!("AURUqAcTZP8mhR6sWVxWyfBbpJRj4A3qqeFzLNhrwayE");

pub mod stake_pool_mints {
    pub mod lido_stsol {
        use anchor_lang::declare_id;
        declare_id!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj");
    }
    pub mod marinade_msol {
        use anchor_lang::declare_id;
        declare_id!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
    }
}

/// [asol] program.
#[program]
pub mod asol {
    use super::*;

    /// Provisions a new aggregate SOL.
    #[access_control(ctx.accounts.validate())]
    pub fn new_aggregate(
        ctx: Context<NewAggregate>,
        agg_bump: u8,
        crate_bump: u8,
    ) -> ProgramResult {
        crate_token::cpi::new_crate(
            CpiContext::new(
                ctx.accounts.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::NewCrate {
                    crate_mint: ctx.accounts.crate_mint.to_account_info(),
                    crate_token: ctx.accounts.crate_token.to_account_info(),
                    fee_to_setter: ctx.accounts.aggregate.to_account_info(),
                    fee_setter_authority: ctx.accounts.aggregate.to_account_info(),
                    author_fee_to: ctx.accounts.aggregate.to_account_info(),
                    issue_authority: ctx.accounts.aggregate.to_account_info(),
                    withdraw_authority: ctx.accounts.redeem_in_kind.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            ),
            crate_bump,
        )?;

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"Aggregate".as_ref(),
            &ctx.accounts.crate_token.key().to_bytes(),
            &[agg_bump],
        ]];

        // Withdraw fee is 0.5% or 50 bps
        crate_token::cpi::set_withdraw_fee(
            CpiContext::new(
                ctx.accounts.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::SetFees {
                    crate_token: ctx.accounts.crate_token.to_account_info(),
                    fee_setter: ctx.accounts.aggregate.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            50,
        )?;

        let aggregate = &mut ctx.accounts.aggregate;
        aggregate.crate_token = ctx.accounts.crate_token.key();
        aggregate.bump = agg_bump;

        aggregate.curator = ctx.accounts.admin.key();
        aggregate.curator_setter = ctx.accounts.admin.key();

        emit!(NewAggregateEvent {
            aggregate: aggregate.key(),
            curator: aggregate.curator,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }

    /// Adds a new stake pool to an aggregate.
    #[access_control(ctx.accounts.validate())]
    pub fn add_stake_pool(
        ctx: Context<AddStakePool>,
        bump: u8,
        accounting_method: AccountingMethod,
    ) -> ProgramResult {
        let aggregate = &ctx.accounts.aggregate;

        let stake_pool = &mut ctx.accounts.stake_pool;
        stake_pool.aggregate = aggregate.key();
        stake_pool.mint = ctx.accounts.mint.key();
        stake_pool.bump = bump;
        stake_pool.accounting_method = accounting_method;

        stake_pool.stats.total_amount_deposited = 0;
        stake_pool.stats.total_amount_minted = ASOL::from(0);

        let aggregate = &mut ctx.accounts.aggregate;
        aggregate.stake_pools.push(StakePoolMeta {
            mint: stake_pool.mint,
            accounting_method,
        });

        emit!(AddStakePoolEvent {
            aggregate: aggregate.key(),
            stake_pool: stake_pool.key(),

            curator: aggregate.curator,
            mint: stake_pool.mint,
            accounting_method,

            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }

    /// Sets the curator.
    #[access_control(ctx.accounts.validate())]
    pub fn set_curator(ctx: Context<SetCurator>) -> ProgramResult {
        let aggregate = &mut ctx.accounts.aggregate;
        let previous_curator = aggregate.curator;
        aggregate.curator = ctx.accounts.next_curator.key();

        emit!(SetCuratorEvent {
            aggregate: aggregate.key(),
            previous_curator,
            curator: aggregate.curator,
            curator_setter: aggregate.curator_setter,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }

    /// Mints aSOL from Lido stSOL.
    #[access_control(ctx.accounts.validate())]
    pub fn mint_lido(ctx: Context<SyncAndMint>, deposit_amount: u64) -> ProgramResult {
        ctx.accounts.sync_and_mint_lido(deposit_amount)
    }

    /// Mints aSOL from Marinade mSOL.
    #[access_control(ctx.accounts.validate())]
    pub fn mint_marinade(ctx: Context<SyncAndMint>, deposit_amount: u64) -> ProgramResult {
        ctx.accounts.sync_and_mint_marinade(deposit_amount)
    }

    /// Getter that logs the prices of all staked SOLs.
    pub fn print_aggregate_info(ctx: Context<SyncAndMint>) -> ProgramResult {
        // ensure not mut
        let accounts: &SyncAndMint = ctx.accounts;
        emit!(AggregateInfoEvent {
            snapshot: accounts.build_snapshot()?,
            timestamp: Clock::get()?.unix_timestamp
        });
        Ok(())
    }
}

// --------------------------------
// Context Structs
// --------------------------------

/// Accounts for [asol::new_aggregate].
#[derive(Accounts)]
#[instruction(agg_bump: u8)]
pub struct NewAggregate<'info> {
    /// Information about the crate.
    #[account(
        init,
        seeds = [
            b"Aggregate".as_ref(),
            crate_token.key().to_bytes().as_ref()
        ],
        bump = agg_bump,
        payer = payer,
        // support up to 30 stake pools for aSOL
        space = 8 + std::mem::size_of::<Aggregate>() + std::mem::size_of::<StakePool>() * MAX_STAKE_POOLS
    )]
    pub aggregate: Account<'info, Aggregate>,

    /// [Mint] of the [crate_token::CrateToken].
    pub crate_mint: Account<'info, Mint>,

    #[account(mut)]
    pub crate_token: UncheckedAccount<'info>,

    /// Redeem in kind.
    pub redeem_in_kind: UncheckedAccount<'info>,

    /// Payer of the crate initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The admin, who becomes the curator and the curator setter.
    pub admin: UncheckedAccount<'info>,

    /// System program.
    pub system_program: Program<'info, System>,

    /// Crate token program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
}

/// Accounts for [asol::add_stake_pool].
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct AddStakePool<'info> {
    #[account(mut)]
    pub aggregate: Account<'info, Aggregate>,

    /// The [StakePool] to add.
    #[account(
        init,
        seeds = [
            b"StakePool",
            aggregate.key().to_bytes().as_ref(),
            mint.key().to_bytes().as_ref()
        ],
        bump = bump,
        payer = payer
    )]
    pub stake_pool: Account<'info, StakePool>,

    /// [Mint] of the stake pool.
    pub mint: Account<'info, Mint>,

    /// The [Aggregate::curator].
    pub curator: Signer<'info>,

    /// Payer of the crate initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program.
    pub system_program: Program<'info, System>,
}

/// Accounts for [asol::set_curator].
#[derive(Accounts)]
pub struct SetCurator<'info> {
    /// [Aggregate].
    #[account(mut)]
    pub aggregate: Account<'info, Aggregate>,
    /// The [Aggregate::curator].
    pub curator_setter: Signer<'info>,
    /// The [Aggregate::curator] to set.
    pub next_curator: UncheckedAccount<'info>,
}

/// Accounts for minting aSOL.
#[derive(Accounts)]
pub struct MintASol<'info> {
    /// Information about the aggregate.
    #[account(mut)]
    pub aggregate: Account<'info, Aggregate>,

    /// The [StakePool].
    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,

    /// [TokenAccount] holding the [StakePool] tokens of the [crate_token::CrateToken].
    #[account(mut)]
    pub stake_pool_tokens: Box<Account<'info, TokenAccount>>,

    /// Information about the crate.
    pub crate_token: Box<Account<'info, crate_token::CrateToken>>,

    /// [Mint] of the [crate_token::CrateToken].
    #[account(mut)]
    pub crate_mint: Box<Account<'info, Mint>>,

    /// The depositor into the pool.
    #[account(mut)]
    pub depositor: Signer<'info>,

    /// The source of the deposited [StakePool] tokens.
    #[account(mut)]
    pub depositor_source: Box<Account<'info, TokenAccount>>,

    /// Destination of the issued tokens.
    #[account(mut)]
    pub mint_destination: Box<Account<'info, TokenAccount>>,

    /// [Token] program.
    pub token_program: Program<'info, Token>,

    /// [crate_token::program::CrateToken] program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
}

#[derive(Accounts)]
pub struct SyncAndMint<'info> {
    /// Mint aSOL
    pub mint_asol: MintASol<'info>,
    /// Sync accounts
    pub sync: SyncAll<'info>,
}

/// Accounts for synchronization.
/// TODO: this should be generic
#[derive(Accounts)]
pub struct SyncAll<'info> {
    /// Marinade accounts.
    pub marinade: SyncMarinade<'info>,

    /// Lido accounts.
    pub lido: SyncLido<'info>,
}

#[derive(Accounts)]
pub struct SyncMarinade<'info> {
    /// [marinade] state account.
    pub marinade: Box<Account<'info, marinade::State>>,

    /// [TokenAccount] holding the tokens of the [StakePool].
    pub marinade_stake_pool_tokens: Box<Account<'info, TokenAccount>>,
}

#[derive(Accounts)]
pub struct SyncLido<'info> {
    /// [lido_anchor] account.
    pub lido: Box<Account<'info, lido_anchor::Lido>>,

    /// [TokenAccount] holding the tokens of the [StakePool].
    pub lido_stake_pool_tokens: Box<Account<'info, TokenAccount>>,
}

/// Errors.
#[error]
pub enum ErrorCode {
    #[msg("Must be curator.")]
    UnauthorizedNotCurator,
    #[msg("Must be curator setter.")]
    UnauthorizedNotCuratorSetter,

    #[msg("Pool not found in snapshot.", offset = 10)]
    PoolNotFoundInSnapshot,
    #[msg("Cannot add a pool that has already been added.")]
    PoolAlreadyAdded,
}
