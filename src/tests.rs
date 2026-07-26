use crate::ID;
use quasar_lang::pda::find_program_address_const;
use quasar_svm::{Account, Pubkey, QuasarSvm};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_amm.so").unwrap();
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), &elf)
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

    fn initialize_instruction(
        payer: Address,
        system_program: Address,
        mint_x: Address,
        mint_y: Address,
    ) -> Instruction {
        let (config_address, _) = find_program_address_const(
            &[b"amm", payer.as_ref(), mint_x.as_ref(), mint_y.as_ref()],
            &ID,
        );

        Instruction {
            program_id: Address::from(crate::ID.to_bytes()),
            accounts: vec![
                AccountMeta::new(payer, true),
                AccountMeta::new(config_address, false),
                AccountMeta::new_readonly(system_program, false),
            ],
            data: [
                vec![0u8],
                mint_x.to_bytes().to_vec(),
                mint_y.to_bytes().to_vec(),
            ]
            .concat(),
        }
    }

    let instruction = initialize_instruction(
        Address::from(payer.to_bytes()),
        Address::from(quasar_svm::system_program::ID.to_bytes()),
        Address::from_str_const("So11111111111111111111111111111111111111112"),
        Address::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
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
        1,
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
        2,
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
        3,
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
