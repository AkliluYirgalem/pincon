use {
    mollusk_svm::{result::Check, Mollusk},
    pinocchio::{error::ProgramError, Address},
    solana_sdk::{
        account,
        instruction::{AccountMeta, Instruction},
    },
};

#[test]
fn test_type_native_system_success() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_type_native_system");

    let user = Address::new_unique();
    let system_account = Address::from_str_const("11111111111111111111111111111111");
    let accounts = vec![
        (user, account::Account::default()),
        (system_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(system_account, false),
        ],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}

#[test]
fn test_type_native_system_failure() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_type_native_system");

    let user = Address::new_unique();
    let system_account = Address::new_unique(); // What are the odds this is the system_account address?
    let accounts = vec![
        (user, account::Account::default()),
        (system_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(system_account, false),
        ],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::IncorrectProgramId)],
    );
}
