use {
    pincon::InstructionAccounts,
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
};

#[derive(InstructionAccounts)]
pub struct Initialize<'view> {
    #[pincon(signer)]
    pub user: &'view AccountView,
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
