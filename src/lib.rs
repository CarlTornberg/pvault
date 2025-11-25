use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey
};
use pinocchio_pubkey::declare_id;

mod processors;
mod instructions;
mod states;
pub use processors::*;
pub use instructions::*;
pub use states::*;

entrypoint!(process_instruction);

declare_id!("pinzWHbUW7wQ16XcWcEjw4fucktuoHCnBfbbGB93PFo");

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
        &initialize::DISCRIMINATOR => 
            { Initialize::try_from((data, accounts))?.process() },
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
