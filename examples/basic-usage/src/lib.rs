use pincon::InstructionAccounts;
use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error};
use pinocchio_log::log;

#[derive(InstructionAccounts)]
pub struct Initialize {
    #[pincon(signer)]
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
