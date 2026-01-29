use pincon::InstructionAccounts;
use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};

#[derive(InstructionAccounts)]
pub struct Initialize {
    #[pincon(mut, signer)]
    pub authority: AccountView,
    pub data_account: AccountView,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
