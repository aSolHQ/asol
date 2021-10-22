use crate::{
    accounting::Accountant, types::SOL, AccountingMethod, MintASol, Snapshot, StakePoolSnapshot,
    SyncAndMint, ASOL,
};
use anchor_lang::prelude::*;
use vipers::{unwrap_int, unwrap_or_err};

impl<'info> SyncAndMint<'info> {
    /// Builds a snapshot of all balances and conversions.
    pub fn build_snapshot(&self) -> Result<Snapshot, ProgramError> {
        let pool_snapshots: Vec<StakePoolSnapshot> = self
            .mint_asol
            .aggregate
            .stake_pools
            .iter()
            .map(|pool| match pool.accounting_method {
                AccountingMethod::Lido => {
                    StakePoolSnapshot::try_from_accountant(pool, &self.sync.lido)
                }
                AccountingMethod::Marinade => {
                    StakePoolSnapshot::try_from_accountant(pool, &self.sync.marinade)
                }
            })
            .collect::<Result<Vec<StakePoolSnapshot>, ProgramError>>()?;

        let balance_sol_u64: u64 = pool_snapshots
            .iter()
            .map(|snap| Ok(unwrap_int!(snap.pool_balance_sol())))
            .sum::<Result<u64, ProgramError>>()?;
        let balance_sol = SOL::from(balance_sol_u64);

        Ok(Snapshot {
            balance_sol,
            supply: ASOL::from(self.mint_asol.crate_mint.supply),
            stake_pools: pool_snapshots,
        })
    }

    pub fn sync_and_mint_lido(&mut self, deposit_amount: u64) -> ProgramResult {
        let snapshot = self.build_snapshot()?;
        self.mint_asol
            .mint_asol(&snapshot, &self.sync.lido, deposit_amount)?;
        Ok(())
    }

    pub fn sync_and_mint_marinade(&mut self, deposit_amount: u64) -> ProgramResult {
        let snapshot = self.build_snapshot()?;
        self.mint_asol
            .mint_asol(&snapshot, &self.sync.marinade, deposit_amount)?;
        Ok(())
    }
}

impl<'info> MintASol<'info> {
    /// Mints aSOL.
    pub fn mint_asol<T: Accountant<'info>>(
        &mut self,
        snapshot: &Snapshot,
        minter: &T,
        deposit_amount: u64,
    ) -> ProgramResult {
        let pool_snapshot = unwrap_or_err!(
            snapshot
                .stake_pools
                .iter()
                .find(|pool| pool.pool_mint == minter.crate_reserves().mint),
            PoolNotFoundInSnapshot
        );

        // ignore zero deposit
        if deposit_amount == 0 {
            return Ok(());
        }

        // compute the amount of tokens to mint
        let deposit_sol_value = minter.sol_value(deposit_amount)?;
        let mint_amount = snapshot.compute_asol_amount_from_sol(deposit_sol_value)?;

        // ignore zero mint
        if mint_amount.amount == 0 {
            return Ok(());
        }

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"Aggregate".as_ref(),
            &self.aggregate.crate_token.to_bytes(),
            &[self.aggregate.bump],
        ]];

        // transfer stake pool tokens to the crate
        anchor_spl::token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: self.depositor_source.to_account_info(),
                    to: self.stake_pool_tokens.to_account_info(),
                    authority: self.depositor.to_account_info(),
                },
            ),
            deposit_amount,
        )?;

        // issue new crate tokens
        crate_token::cpi::issue(
            CpiContext::new_with_signer(
                self.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::Issue {
                    crate_token: self.crate_token.to_account_info(),
                    crate_mint: self.crate_mint.to_account_info(),
                    issue_authority: self.aggregate.to_account_info(),
                    mint_destination: self.mint_destination.to_account_info(),

                    // there are no author/protocol fees, so we pass in garbage here
                    author_fee_destination: self.mint_destination.to_account_info(),
                    protocol_fee_destination: self.mint_destination.to_account_info(),

                    token_program: self.token_program.to_account_info(),
                },
                signer_seeds,
            ),
            mint_amount.amount,
        )?;

        // update stats
        let stake_pool_state = &mut self.stake_pool;
        stake_pool_state.stats.total_amount_deposited = unwrap_int!(stake_pool_state
            .stats
            .total_amount_deposited
            .checked_add(deposit_amount));
        stake_pool_state.stats.total_amount_minted = ASOL::from(unwrap_int!(stake_pool_state
            .stats
            .total_amount_minted
            .amount
            .checked_add(mint_amount.amount)));

        // record snapshot
        stake_pool_state.latest_snapshot.aggregate_balance_sol = snapshot.balance_sol;
        stake_pool_state.latest_snapshot.aggregate_supply = snapshot.supply;
        stake_pool_state.latest_snapshot.snapshot = *pool_snapshot;
        stake_pool_state.latest_snapshot.snapshot_ts = Clock::get()?.unix_timestamp;

        // record aggregate snapshot
        let aggregate = &mut self.aggregate;
        aggregate.latest_snapshot = snapshot.clone();
        let now = Clock::get()?.unix_timestamp;
        aggregate.latest_snapshot_ts = now;

        // emit event
        emit!(crate::MintASolEvent {
            depositor: self.depositor.key(),
            stake_pool_mint: self.depositor_source.mint,
            accounting_method: T::METHOD,
            deposit_amount,
            mint_amount: mint_amount.amount,
            timestamp: now
        });

        Ok(())
    }
}
