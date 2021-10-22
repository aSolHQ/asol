use vipers::{assert_keys, unwrap_int};

use crate::{
    accounting::Accountant, StakePoolMeta, ASOL, MIN_LIQUIDITY_FOR_EXACT_CALCULATION, SOL,
};
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use num_traits::ToPrimitive;

/// A balance snapshot.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// SOL value of the pool's balance at the time of the snapshot.
    pub balance_sol: SOL,
    /// Total supply.
    pub supply: ASOL,
    /// Stake pools.
    pub stake_pools: Vec<StakePoolSnapshot>,
}

/// A balance snapshot of a stake pool.
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct StakePoolSnapshot {
    /// Mint of the pool.
    pub pool_mint: Pubkey,
    /// Amount of stake pool tokens in the pool.
    pub pool_balance: u64,
    /// Amount of SOL received for 1e9 tokens. (Price)
    pub sol_for_1e9: SOL,
}

impl Snapshot {
    /// Gets the number of [ASOL] corresponding to an amount of [SOL].
    pub fn compute_asol_amount_from_sol(&self, sol_amount: SOL) -> Result<ASOL, ProgramError> {
        // if less than 1 SOL is staked, the price is equal to SOL price.
        // this is to avoid precision errors with tiny balances.
        if self.balance_sol.amount <= MIN_LIQUIDITY_FOR_EXACT_CALCULATION {
            return Ok(ASOL::from(sol_amount.amount));
        }
        Ok(unwrap_int!(
            sol_amount.checked_mul_asol(self.supply, self.balance_sol)
        ))
    }
}

impl StakePoolSnapshot {
    /// The [SOL] value of the pool's balance, based on the price.
    pub fn pool_balance_sol(&self) -> Option<u64> {
        self.sol_for_1e9
            .to_u128()
            .checked_mul(self.pool_balance.into())?
            .checked_div(LAMPORTS_PER_SOL.into())?
            .to_u64()
    }

    /// Creates a pool snapshot from an [Accountant].
    pub fn try_from_accountant<'info, T: Accountant<'info>>(
        pool: &StakePoolMeta,
        accountant: &T,
    ) -> Result<StakePoolSnapshot, ProgramError> {
        assert_keys!(
            pool.mint,
            accountant.crate_reserves().mint,
            format!("incorrect pool mint for {:?}", T::METHOD)
        );
        Self::try_from_accountant_unchecked(accountant)
    }

    /// Creates a pool snapshot from an accountant.
    fn try_from_accountant_unchecked<'info, T: Accountant<'info>>(
        accountant: &T,
    ) -> Result<StakePoolSnapshot, ProgramError> {
        let reserves = accountant.crate_reserves();
        Ok(StakePoolSnapshot {
            pool_mint: reserves.mint,
            pool_balance: reserves.amount,
            sol_for_1e9: accountant.sol_value(LAMPORTS_PER_SOL)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{stake_pool_mints::*, *};
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

    #[test]
    fn test_compute_output_mint_amount() {
        let agg = Snapshot {
            balance_sol: SOL::from(2_200_000),
            supply: ASOL::from(2_000_000),
            stake_pools: vec![
                StakePoolSnapshot {
                    pool_mint: lido_stsol::ID,
                    pool_balance: 1_000_000,
                    sol_for_1e9: SOL::from(1_100_000),
                },
                StakePoolSnapshot {
                    pool_mint: marinade_msol::ID,
                    pool_balance: 1_000_000,
                    sol_for_1e9: SOL::from(1_100_000),
                },
            ],
        };
        let output = agg
            .compute_asol_amount_from_sol(SOL::from(1_100_000))
            .unwrap();

        // it's below the minimum
        assert_eq!(output, ASOL::from(1_100_000));
    }

    #[test]
    fn test_compute_output_mint_amount_imbalanced() {
        let agg = Snapshot {
            balance_sol: SOL::from(3_300_000),
            supply: ASOL::from(3_000_000),
            stake_pools: vec![
                StakePoolSnapshot {
                    pool_mint: lido_stsol::ID,
                    pool_balance: 1_000_000,
                    sol_for_1e9: SOL::from(1_100_000),
                },
                StakePoolSnapshot {
                    pool_mint: marinade_msol::ID,
                    pool_balance: 1_000_000,
                    sol_for_1e9: SOL::from(2_200_000),
                },
            ],
        };
        let output = agg
            .compute_asol_amount_from_sol(SOL::from(1_100_000))
            .unwrap();

        // it's below the minimum
        assert_eq!(output, ASOL::from(1_100_000));
    }

    #[test]
    fn test_compute_output_mint_amount_above_minimum_imbalanced() {
        let agg = Snapshot {
            balance_sol: SOL::from(3_300_000_000),
            supply: ASOL::from(3_000_000_000),
            stake_pools: vec![
                StakePoolSnapshot {
                    pool_mint: lido_stsol::ID,
                    pool_balance: 1_000_000_000,
                    sol_for_1e9: SOL::from(1_100_000),
                },
                StakePoolSnapshot {
                    pool_mint: marinade_msol::ID,
                    pool_balance: 1_000_000_000,
                    sol_for_1e9: SOL::from(2_200_000),
                },
            ],
        };
        let output = agg
            .compute_asol_amount_from_sol(SOL::from(1_100_000))
            .unwrap();

        // it's below the minimum
        assert_eq!(output, ASOL::from(1_000_000));
    }

    #[test]
    fn test_pool_balance_sol_empty() {
        let snap = StakePoolSnapshot {
            pool_balance: 0,
            sol_for_1e9: SOL::from(LAMPORTS_PER_SOL),
            ..Default::default()
        };
        assert_eq!(snap.pool_balance_sol().unwrap(), 0);
    }

    #[test]
    fn test_pool_balance_sol_nonempty() {
        let snap = StakePoolSnapshot {
            pool_balance: 1_000_000,
            sol_for_1e9: SOL::from(LAMPORTS_PER_SOL),
            ..Default::default()
        };
        assert_eq!(snap.pool_balance_sol().unwrap(), 1_000_000);
    }

    #[test]
    fn test_pool_balance_sol_large() {
        let snap = StakePoolSnapshot {
            pool_balance: 1_000_000,
            sol_for_1e9: SOL::from(1_100_000_000),
            ..Default::default()
        };
        assert_eq!(snap.pool_balance_sol().unwrap(), 1_100_000);
    }

    #[test]
    fn test_pool_balance_zero_price() {
        let snap = StakePoolSnapshot {
            pool_balance: 1_000_000,
            sol_for_1e9: SOL::from(0),
            ..Default::default()
        };
        assert_eq!(snap.pool_balance_sol().unwrap(), 0);
    }
}
