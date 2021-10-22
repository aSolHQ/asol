//! Wrapper around the [lido] program.
use std::ops::Deref;

use anchor_lang::prelude::*;

declare_id!("CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi");

mod solido_account {
    use anchor_lang::declare_id;

    declare_id!("49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn");
}

/// Solido account
pub static SOLIDO_ACCOUNT: Pubkey = solido_account::ID;

#[derive(Clone, Debug, Default)]
pub struct Lido(lido::state::Lido);

impl Owner for Lido {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl Deref for Lido {
    type Target = lido::state::Lido;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AccountSerialize for Lido {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ProgramError> {
        AnchorSerialize::serialize(&self.0, writer).map_err(|_| ProgramError::InvalidAccountData)
    }
}

impl AccountDeserialize for Lido {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Self::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        let result: lido::state::Lido = AnchorDeserialize::deserialize(buf)?;
        Ok(Lido(result))
    }
}
