use {
    pincon::InstructionAccounts,
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
};

#[derive(InstructionAccounts)]
pub struct Noop<'view> {
    #[pincon(signer)]
    pub user: &'view AccountView,
    #[pincon(type = native(stake))]
    pub stake_account: &'view AccountView,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let _ctx = Noop::try_from(accounts)?;
    Ok(())
}
