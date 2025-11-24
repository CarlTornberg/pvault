use pinocchio::{account_info::AccountInfo, program_error::ProgramError, sysvars::{rent, Sysvar}, ProgramResult};
use pinocchio_system::instructions::CreateAccount;

use crate::{Instruction, Vault};

pub type InitializeProc<'a> = Instruction<'a, InitializeAccounts<'a>, InitializeData<'a>>;

pub struct InitializeAccounts<'a> {
    /// 0: [w]: Initialized, Allocated, Assigned, Signer, writable, payer,
    pub authority: &'a AccountInfo,

    /// 1: [w]: Not initialized
    pub vault_data: &'a AccountInfo,

    /// 2: [w]: Not initialized
    pub vault: &'a AccountInfo,
}

#[repr(C)]
pub struct InitializeData<'a> {
    pub locked: &'a bool,
}

impl<'a> Instruction<'a, InitializeAccounts<'a>, InitializeData<'a>> {
    pub const DISCRIMINATOR:&'a u8 = &0;

    pub fn process(&self) -> ProgramResult {
            
        let space = core::mem::size_of::<Vault>();
        CreateAccount {
            from: self.accounts.authority,
            to: self.accounts.vault_data,
            space: space as u64,
            lamports: rent::Rent::get()?.minimum_balance(space),
            owner: &pinocchio_system::ID,
        }
        .invoke()?;

        Ok(())
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Instruction<'a, InitializeAccounts<'a>, InitializeData<'a>>  {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;

        // Check accounts
        if accounts.len() < 3 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
    
        // Authority 
        let authority = &accounts[0];
        if !authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        // Vault data 
        let vault_data = &accounts[1];
        if !vault_data.is_writable() {
            return Err(ProgramError::Custom(0));
        }
        if vault_data.lamports() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // Vault 
        let vault = &accounts[2];
        if !vault.is_writable() {
            return Err(ProgramError::Custom(0));
        }
        if vault.lamports() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // Check data
        if data.len() != core::mem::size_of::<bool>() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let locked = unsafe { &*(data.as_ptr() as *const bool) };

        Ok(Self {
            discriminator: Self::DISCRIMINATOR,
            accounts: InitializeAccounts {
                authority, vault_data, vault,
            },
            data: InitializeData{ locked },
        })
    }
}
