use anchor_lang::prelude::*;
use num_traits::ToPrimitive;

/// An amount of SOL.
#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct SOL {
    pub amount: u64,
}

/// An amount of aSOL.
#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ASOL {
    pub amount: u64,
}

impl SOL {
    /// Converts to u128.
    pub fn to_u128(&self) -> u128 {
        self.amount as u128
    }

    /// Converts the [SOL] amount to [ASOL].
    pub fn checked_mul_asol(&self, numerator: ASOL, denominator: SOL) -> Option<ASOL> {
        Some(ASOL::from(
            (self.to_u128())
                .checked_mul(numerator.to_u128())
                .and_then(|v| v.checked_div(denominator.to_u128()))
                .and_then(|v| v.to_u64())?,
        ))
    }
}

impl ASOL {
    /// Converts to u128.
    pub fn to_u128(&self) -> u128 {
        self.amount as u128
    }
}

impl From<SOL> for u128 {
    fn from(sol: SOL) -> Self {
        sol.to_u128()
    }
}

impl From<ASOL> for u128 {
    fn from(asol: ASOL) -> Self {
        asol.to_u128()
    }
}

impl From<u64> for SOL {
    fn from(amount: u64) -> Self {
        SOL { amount }
    }
}

impl From<u64> for ASOL {
    fn from(amount: u64) -> Self {
        ASOL { amount }
    }
}
