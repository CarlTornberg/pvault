use pinocchio::{
    account_info::AccountInfo, entrypoint, msg, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};
use pinocchio_pubkey::declare_id;
mod handlers;

entrypoint!(process_instruction);

declare_id!("11111111111111111111111111111111");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &ID { return Err(ProgramError::IncorrectProgramId); }

    let [inst, data @ ..] = instruction_data else {
        return Err(ProgramError::InvalidInstructionData);
    };

    match inst {
        0 => {},
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}
