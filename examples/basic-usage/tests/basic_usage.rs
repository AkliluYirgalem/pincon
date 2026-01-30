use mollusk_svm::{program, result::Check, Mollusk};
use pinocchio::Address;
use pinocchio_system;
use solana_sdk::{
    account,
    instruction::{AccountMeta, Instruction},
};

#[test]
fn test_basic_usage() {
    let program_id = Address::new_unique();
    let mollusk = Mollusk::new(&program_id, "../output/basic_usage");

    let authority = Address::new_unique();
    let data_account = Address::new_unique();

    let accounts = vec![
        (authority, account::Account::default()),
        (data_account, account::Account::default()),
        program::keyed_account_for_system_program(),
    ];

    let instruction = Instruction {
        program_id: program_id,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(data_account, false),
            AccountMeta::new(pinocchio_system::ID, false),
        ],
        data: Vec::new(),
    };

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}
