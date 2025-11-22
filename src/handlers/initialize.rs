use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

#[repr(C)]
pub struct InitializeVault {
    pub authority: AccountInfo,
    pub locked: bool,
    pub lamports: u64,
}

impl<'a> TryFrom<&'a &[u8]> for &'a InitializeVault {
    type Error = ProgramError;

    fn try_from(data:&'a &[u8]) -> Result<Self, Self::Error> {
        if data.len() != core::mem::size_of::<Self>() {
            return Err(ProgramError::InvalidInstructionData);
        } 

        Ok( unsafe { &*(data.as_ptr() as *const Self) } )
    }
    
}

