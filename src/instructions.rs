use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

pub enum Instruction<'a> {
    /// 1. [w] Authority: Allocated & Assigned
    /// 2. [w] Vault: Not allocated & Not Assigned
    Initialize {},

    /// 1. [w] Authority: Allocated & Assigned
    /// 2. [w] Vault: Allocated & Assigned
    Deposit {amount: &'a u64},

    /// 1. [w] Authority: Allocated & Assigned
    /// 2. [w] Vault: Allocated & Assigned
    Lock {lock: &'a bool},
    
    /// 1. [w] Authority: Allocated & Assigned
    /// 2. [w] Vault: Allocated & Assigned
    Withdraw {amount: &'a u64},
    
    /// 1. [w] Authority: Allocated & Assigned
    /// 2. [w] Vault: Allocated & Assigned
    Stake {amount: &'a u64, to: &'a Pubkey},
}

impl<'a> TryFrom<&'a &[u8]> for Instruction<'a> {
    type Error = ProgramError;
    fn try_from(value: &'a &[u8]) -> Result<Self, Self::Error> {
        let [discriminator, _data @ ..] = value else {
            return Err(ProgramError::InvalidInstructionData);
        };
        match *discriminator {
            0 => Ok(Instruction::Initialize {  }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

