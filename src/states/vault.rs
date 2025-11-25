use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::{self, Pubkey}};

#[repr(C)]
pub struct Vault {
    pub authority: AccountInfo,
    pub locked: bool,
    pub lamports: u64,
    pub bump: u8,
}

impl Vault {
    pub const DISCRIMINATOR: u8 = 0;

    pub const VAULT_SEED: &[u8] = b"vault";
    pub const VAULT_DATA_SEED: &[u8] = b"vault_data";

    /// Discriminator + data size
    pub const ON_CHAIN_SIZE: usize = core::mem::size_of::<u8>() + core::mem::size_of::<Self>();

    pub fn get_vault_pda(authority: &Pubkey) -> (Pubkey, u8) {
        pubkey::find_program_address(&[Self::VAULT_SEED, authority], &crate::ID)
    }

    pub fn get_vault_data_pda(authority: &Pubkey) -> (Pubkey, u8) {
        pubkey::find_program_address(&[Self::VAULT_DATA_SEED, authority], &crate::ID)
    }
}


impl<'a> TryFrom<&'a &[u8]> for &'a Vault {
    type Error = ProgramError;

    fn try_from(value:&'a &[u8]) -> Result<Self, Self::Error> {
        let [discriminator, data @ .. ] = value else {
            return Err(ProgramError::InvalidInstructionData);
        };

        if *discriminator != Vault::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }

        if data.len() != core::mem::size_of::<Self>() {
            return Err(ProgramError::InvalidInstructionData);
        } 

        Ok( unsafe { &*(data.as_ptr() as *const Self) } )
    }
}

