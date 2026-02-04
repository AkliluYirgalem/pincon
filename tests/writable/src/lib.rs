use {
    pincon::InstructionAccounts,
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
};

#[derive(InstructionAccounts)]
pub struct CreatePDA<'view> {
    #[pincon(signer)]
    pub authority: &'view AccountView,
    #[pincon(mut)]
    pub new_account: &'view AccountView,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let _ctx = CreatePDA::try_from(accounts)?;
    Ok(())
}
