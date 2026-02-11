use {
    mollusk_svm::{result::Check, Mollusk},
    pinocchio::{error::ProgramError, Address},
    solana_sdk::{
        account,
        instruction::{AccountMeta, Instruction},
    },
};

#[test]
fn test_writable_success() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_writable");

    let authority = Address::new_unique();
    let new_account = Address::new_unique();
    let accounts = vec![
        (authority, account::Account::default()),
        (new_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(new_account, false),
        ],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}

#[test]
fn test_writable_failure() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_writable");

    let authority = Address::new_unique();
    let new_account = Address::new_unique();
    let accounts = vec![
        (authority, account::Account::default()),
        (new_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(new_account, false),
        ],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Immutable)],
    );
}
