use {
    mollusk_svm::{result::Check, Mollusk},
    pinocchio::{error::ProgramError, Address},
    solana_sdk::{
        account,
        instruction::{AccountMeta, Instruction},
    },
};

#[test]
fn test_constraint_program_config_success() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(
        &program_id,
        "../../target/deploy/test_constraint_program_config",
    );

    let user = Address::new_unique();
    let vote_account = Address::from_str_const("Config1111111111111111111111111111111111111");
    let accounts = vec![
        (user, account::Account::default()),
        (vote_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(vote_account, false),
        ],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}

#[test]
fn test_constraint_program_config_failure() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(
        &program_id,
        "../../target/deploy/test_constraint_program_config",
    );

    let user = Address::new_unique();
    let vote_account = Address::new_unique();
    let accounts = vec![
        (user, account::Account::default()),
        (vote_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(vote_account, false),
        ],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::IncorrectProgramId)],
    );
}
