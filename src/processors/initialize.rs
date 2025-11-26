use pinocchio::{ProgramResult, account_info::AccountInfo, instruction::Signer, msg, program_error::ProgramError, pubkey, seeds, sysvars::{Sysvar, rent::Rent}};
use pinocchio_system::instructions::CreateAccount;

use crate::Vault;

pub const DISCRIMINATOR: u8 = 0;

pub struct Initialize<'a> {
    pub accounts: InitializeAccounts<'a>,
    pub instruction_data: InitializeInstructionData<'a>,
}

pub struct InitializeAccounts<'a> {
    /// 0: [w]: Initialized, Allocated, Assigned, Signer, writable, payer,
    pub authority: &'a AccountInfo,

    /// 1: [w]: Not initialized
    pub vault_data: &'a AccountInfo,
    pub vault_data_bump: u8,

    /// 2: [w]: Not initialized
    pub vault: &'a AccountInfo,
    pub vault_bump: u8,
}

#[repr(C)]
pub struct InitializeInstructionData<'a> {
    pub locked: &'a bool,
}

impl<'a> InitializeInstructionData<'a> {
    /// # Safety
    pub unsafe fn pack(&self) -> &[u8] {
        core::slice::from_raw_parts(
            (self as *const Self) as *const u8,
            core::mem::size_of::<Self>()
        )
    }
    
    pub fn get_packed_instruction_data(locked: bool) -> [u8; 2] {
        [DISCRIMINATOR, locked as u8]
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Initialize<'a> {
    type Error = ProgramError;
    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts_slice) = value;
        if accounts_slice.len() < 3 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let authority = &accounts_slice[0];
        let vault_data = &accounts_slice[1];
        let vault = &accounts_slice[2];

        if !authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let (vault_data_pda, vault_data_bump) = Vault::get_vault_data_pda(authority.key());
        if !pubkey::pubkey_eq(vault_data.key(), &vault_data_pda) {
            return Err(ProgramError::InvalidSeeds);
        }

        if vault_data.lamports() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let (vault_pda, vault_bump) = Vault::get_vault_pda(authority.key());
        if !pubkey::pubkey_eq(vault.key(), &vault_pda) {
            return Err(ProgramError::InvalidSeeds);
        }
        if vault.lamports() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        let accounts = InitializeAccounts { 
            authority, 
            vault_data, 
            vault_data_bump, 
            vault, 
            vault_bump 
        };

        // Data
        if data.len() != core::mem::size_of::<bool>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let locked = unsafe { &*(data.as_ptr() as *const bool) }; 
        let instruction_data = InitializeInstructionData{ locked };

        Ok( Self{ accounts, instruction_data}) 
    }
}

impl<'a> Initialize<'a> {
    pub fn process(&'a self) -> ProgramResult {
        let space = core::mem::size_of::<InitializeInstructionData>();
        let rent = Rent::get()?.minimum_balance(space);

        let data_bump = &[self.accounts.vault_data_bump];
        let vault_data_seeds = seeds!(
            Vault::VAULT_DATA_SEED, 
            self.accounts.authority.key(), 
            data_bump
        );
        CreateAccount {
            from: self.accounts.authority,
            to: self.accounts.vault_data,
            lamports: rent,
            space: space as u64,
            owner: &crate::ID,
        }.invoke_signed(&[Signer::from(&vault_data_seeds)])?;
        msg!("Vault data created");

        let vault_bump = &[self.accounts.vault_bump];
        let vault_seeds = seeds!(
            Vault::VAULT_SEED, 
            self.accounts.authority.key(),
            vault_bump
        );
        CreateAccount {
            from: self.accounts.authority,
            to: self.accounts.vault,
            lamports: Rent::get()?.minimum_balance(0),
            space: 0,
            owner: &pinocchio_system::ID,
        }.invoke_signed(&[Signer::from(&vault_seeds)])?;
        msg!("Vault created");

        if let Ok(mut data) = self.accounts.vault_data.try_borrow_mut_data() {
            let new_data = unsafe { self.instruction_data.pack() };
            data.copy_from_slice(new_data);
            msg!("Vault data set");
        }
        else { return Err(ProgramError::AccountBorrowFailed); }

        Ok(())
    }
}
