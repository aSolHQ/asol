//! Validate accounts

use anchor_lang::prelude::*;
use vipers::{assert_keys, invariant};

use crate::{
    stake_pool_mints, AddStakePool, MintASol, NewAggregate, SetCurator, SyncAll, SyncAndMint,
    SyncLido, SyncMarinade, LAMPORTS_DECIMALS,
};
use vipers::validate::Validate;

impl<'info> Validate<'info> for NewAggregate<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(
            self.redeem_in_kind,
            crate_redeem_in_kind::WITHDRAW_AUTHORITY_ADDRESS,
            "redeem_in_kind"
        );
        invariant!(self.crate_mint.supply == 0, "supply must be zero");
        invariant!(
            self.crate_mint.decimals == LAMPORTS_DECIMALS,
            "decimals should be 9"
        );
        Ok(())
    }
}

impl<'info> Validate<'info> for AddStakePool<'info> {
    fn validate(&self) -> ProgramResult {
        require!(
            self.curator.key() == self.aggregate.curator,
            UnauthorizedNotCurator
        );
        require!(
            !self
                .aggregate
                .stake_pools
                .iter()
                .any(|pool| pool.mint == self.mint.key()),
            PoolAlreadyAdded
        );
        Ok(())
    }
}

impl<'info> Validate<'info> for SetCurator<'info> {
    fn validate(&self) -> ProgramResult {
        require!(
            self.aggregate.curator_setter == self.curator_setter.key(),
            UnauthorizedNotCuratorSetter
        );
        Ok(())
    }
}

impl<'info> Validate<'info> for SyncAll<'info> {
    fn validate(&self) -> ProgramResult {
        self.lido.validate()?;
        self.marinade.validate()?;
        Ok(())
    }
}

impl<'info> Validate<'info> for SyncLido<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(*self.lido, lido_anchor::SOLIDO_ACCOUNT, "lido");
        // redundant since it's already validated by being in the list
        assert_keys!(
            self.lido_stake_pool_tokens.mint,
            stake_pool_mints::lido_stsol::ID,
            "lido_stake_pool_tokens.mint"
        );
        Ok(())
    }
}

impl<'info> Validate<'info> for SyncMarinade<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(*self.marinade, marinade::main_state::ID, "marinade");
        // redundant since it's already validated by being in the list
        assert_keys!(
            self.marinade_stake_pool_tokens.mint,
            stake_pool_mints::marinade_msol::ID,
            "marinade_stake_pool_tokens.mint"
        );
        Ok(())
    }
}

impl<'info> Validate<'info> for MintASol<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(
            self.stake_pool.aggregate,
            self.aggregate,
            "stake_pool.aggregate"
        );
        assert_keys!(
            self.stake_pool_tokens.mint,
            self.stake_pool.mint,
            "stake_pool_tokens.mint",
        );
        assert_keys!(
            *self.crate_token,
            self.stake_pool_tokens.owner,
            "crate_token should be stake_pool_tokens.owner"
        );
        assert_keys!(self.crate_token.mint, *self.crate_mint, "crate_token.mint");
        assert_keys!(
            self.mint_destination.mint,
            self.crate_token.mint,
            "mint_destination.mint"
        );

        assert_keys!(
            self.depositor_source.mint,
            self.stake_pool_tokens.mint,
            "depositor_source.mint"
        );
        assert_keys!(
            self.depositor_source.owner,
            self.depositor,
            "depositor_source.owner"
        );
        Ok(())
    }
}

impl<'info> Validate<'info> for SyncAndMint<'info> {
    fn validate(&self) -> ProgramResult {
        self.sync.validate()?;
        self.mint_asol.validate()?;
        Ok(())
    }
}
