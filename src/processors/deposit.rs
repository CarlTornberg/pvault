use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey};

use crate::Vault;

pub const DISCRIMINATOR: u8 = 0;

pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData<'a>,
}

pub struct DepositAccounts<'a> {
    /// 0: [w]: Initialized, Allocated, Assigned, Signer, writable, payer,
    pub authority: &'a AccountInfo,

    /// 1: [w]: Not initialized
    pub vault_data: &'a AccountInfo,

    /// 2: [w]: Not initialized
    pub vault: &'a AccountInfo,
}

#[repr(C)]
pub struct DepositInstructionData<'a> {
    pub amount: &'a u64,
}

impl<'a> DepositInstructionData<'a> {
    /// # Safety
    pub unsafe fn pack(&self) -> &[u8] {
        core::slice::from_raw_parts(
            (self as *const Self) as *const u8,
            core::mem::size_of::<Self>()
        )
    }
    
    pub fn get_packed_instruction_data(amount: u64) -> [u8; 8] {
        amount.to_le_bytes()
    }
}

impl<'a> Deposit<'a> {
    pub fn process(&'a self) -> ProgramResult {
        Ok(())
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;
    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts_slice) = value;
        if accounts_slice.len() < 3 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let accounts = DepositAccounts {
            authority: &accounts_slice[0],
            vault_data: &accounts_slice[1],
            vault: &accounts_slice[2],
        };

        if !accounts.authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }


        if !pubkey::pubkey_eq(accounts.vault_data.key(), &Vault::get_vault_data_pda(accounts.authority.key()).0) {
            return Err(ProgramError::InvalidSeeds);
        }
        if !accounts.vault_data.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let instruction_data = DepositInstructionData {
            amount: &0,
        };
        
        Ok(Deposit { accounts, instruction_data })
    }
}

