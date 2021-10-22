use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use num_traits::cast::ToPrimitive;
use vipers::unwrap_int;

use crate::{state::AccountingMethod, SyncLido, SyncMarinade, SOL};

/// Can account for the amount of SOL in a stake pool.
pub trait Accountant<'info> {
    /// The accounting method to use for the stake pool.
    const METHOD: AccountingMethod;

    /// Calculates the value of the stake pool token amount in SOL.
    fn sol_value(&self, amount: u64) -> Result<SOL, ProgramError>;

    /// Gets the [TokenAccount] of stake pool tokens associated with the Crate.
    fn crate_reserves(&self) -> &TokenAccount;
}

impl<'info> Accountant<'info> for SyncMarinade<'info> {
    const METHOD: AccountingMethod = AccountingMethod::Marinade;

    fn sol_value(&self, amount: u64) -> Result<SOL, ProgramError> {
        let msol_price: u64 = self.marinade.msol_price;
        let sol_value = unwrap_int!((amount as u128)
            .checked_mul(msol_price.into())
            .and_then(|v| v.checked_div(0x1_0000_0000_u128))
            .and_then(|v| v.to_u64()));
        Ok(SOL::from(sol_value))
    }

    fn crate_reserves(&self) -> &TokenAccount {
        &self.marinade_stake_pool_tokens
    }
}

impl<'info> Accountant<'info> for SyncLido<'info> {
    const METHOD: AccountingMethod = AccountingMethod::Lido;

    fn sol_value(&self, amount: u64) -> Result<SOL, ProgramError> {
        let lido = &*self.lido;
        let sol_value = unwrap_int!((amount as u128)
            .checked_mul(lido.exchange_rate.sol_balance.0.into())
            .and_then(|v| v.checked_div(lido.exchange_rate.st_sol_supply.0.into()))
            .and_then(|v| v.to_u64()));
        Ok(SOL::from(sol_value))
    }

    fn crate_reserves(&self) -> &TokenAccount {
        &self.lido_stake_pool_tokens
    }
}
