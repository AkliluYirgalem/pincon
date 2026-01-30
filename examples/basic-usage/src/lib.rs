use pincon::InstructionAccounts;
use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};

#[derive(InstructionAccounts)]
pub struct Initialize<'view> {
    #[pincon(mut, signer)]
    pub authority: &'view AccountView,
    #[pincon(mut)]
    pub data_account: &'view AccountView,
    pub system_account: &'view AccountView,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let _ctx = Initialize::try_from(accounts)?;

    // log!(_)

    Ok(())
}
