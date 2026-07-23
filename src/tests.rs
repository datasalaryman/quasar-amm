use quasar_svm::{Account, Pubkey, QuasarSvm};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_amm.so").unwrap();
    QuasarSvm::new()
        .with_program(&Pubkey::from(crate::ID), &elf)
}

fn instruction_builder(payer: Address, system_program: Address, disciminator: u8) -> Instruction {
    Instruction {
        program_id: Address::from(crate::ID.to_bytes()),
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
        ],
        data: vec![disciminator],
    }
}

#[test]
fn test_initialize() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();

    let instruction = instruction_builder(
        Address::from(payer.to_bytes()),
        Address::from(quasar_svm::system_program::ID.to_bytes()),
        0
    );

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: payer,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();
}

#[test]
fn test_deposit() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();

    let instruction = instruction_builder(
        Address::from(payer.to_bytes()),
        Address::from(quasar_svm::system_program::ID.to_bytes()),
        1
    );

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: payer,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();
}

#[test]
fn test_withdraw() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();

    let instruction = instruction_builder(
        Address::from(payer.to_bytes()),
        Address::from(quasar_svm::system_program::ID.to_bytes()),
        2
    );

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: payer,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();
}

#[test]
fn test_swap() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();

    let instruction = instruction_builder(
        Address::from(payer.to_bytes()),
        Address::from(quasar_svm::system_program::ID.to_bytes()),
        3
    );

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: payer,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();
}
