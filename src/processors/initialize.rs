use pinocchio::{ProgramResult, account_info::AccountInfo, instruction::{AccountMeta, Instruction}, program_error::ProgramError, pubkey::Pubkey, sysvars::{Sysvar, rent::Rent}};
use pinocchio_system::instructions::CreateAccount;

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

    /// 2: [w]: Not initialized
    pub vault: &'a AccountInfo,
}

impl<'a> InitializeAccounts<'a> {
    pub fn as_array(&'a self) -> [&'a Pubkey; 3] {
        [self.authority.key(), self.vault_data.key(), self.vault.key()]
    }
}

#[repr(C)]
pub struct InitializeInstructionData<'a> {
    pub locked: &'a bool,
}

impl<'a> InitializeInstructionData<'a> {
    pub fn pack(&'a self) -> [u8; 2] {
        [DISCRIMINATOR, *self.locked as u8]
    }
}

impl<'a> Initialize<'a> {
    pub fn process(&'a self) -> ProgramResult {

        let space = core::mem::size_of::<InitializeInstructionData>();
        let rent = Rent::get()?.minimum_balance(space);

        CreateAccount {
            from: self.accounts.authority,
            to: self.accounts.vault_data,
            lamports: rent,
            space: space as u64,
            owner: &pinocchio_system::ID,
        }.invoke()?;
        
        Ok(())
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Initialize<'a> {
    type Error = ProgramError;
    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts_slice) = value;
        if accounts_slice.len() < 3 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let accounts = InitializeAccounts{
            authority: &accounts_slice[0],
            vault_data: &accounts_slice[1],
            vault: &accounts_slice[2],
        };

        if !accounts.authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if accounts.vault_data.lamports() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if accounts.vault.lamports() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // Data
        if data.len() != core::mem::size_of::<bool>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let locked = unsafe { &*(data.as_ptr() as *const bool) }; 
        let instruction_data = InitializeInstructionData{ locked };

        Ok( Self{ accounts, instruction_data}) 
    }
}

