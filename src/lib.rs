use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};
use pinocchio_pubkey::declare_id;

mod processors;
mod states;
mod utils;
pub use processors::*;
pub use states::*;
pub use utils::*;

entrypoint!(process_instruction);

declare_id!("11111111111111111111111111111111");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &ID { return Err(ProgramError::IncorrectProgramId); }

    let [discr, data @ ..] = instruction_data else {
        return Err(ProgramError::InvalidInstructionData);
    };

    match discr {
        InitializeProc::DISCRIMINATOR => 
            { InitializeProc::try_from((data, accounts))?.process() },
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
