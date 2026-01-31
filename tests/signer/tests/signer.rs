use {
    mollusk_svm::{result::Check, Mollusk},
    pinocchio::{error::ProgramError, Address},
    solana_sdk::{
        account,
        instruction::{AccountMeta, Instruction},
    },
};

#[test]
fn test_signer_success() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_signer");

    let authority = Address::new_unique();
    let accounts = vec![(authority, account::Account::default())];

    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(authority, true)],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}

#[test]
fn test_signer_failure() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_signer");

    let authority = Address::new_unique();
    let accounts = vec![(authority, account::Account::default())];

    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(authority, false)],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}
