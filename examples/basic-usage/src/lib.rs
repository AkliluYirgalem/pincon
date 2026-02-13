use {
    pincon::InstructionAccounts,
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
};

#[derive(InstructionAccounts)]
struct Initialize<'view> {
    #[pincon(mut, signer)]
    pub authority: &'view AccountView,
    #[pincon(mut)]
    pub data_account: &'view AccountView,
    #[pincon(program = token)]
    pub token_program: &'view AccountView,
    #[pincon(program = system)]
    pub system_account: &'view AccountView,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let _ctx = Initialize::try_from(accounts)?;

    Ok(())
}
