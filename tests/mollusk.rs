// #![cfg(feature = "test-sbf")]

#[allow(unused_imports)]
#[cfg(test)]
mod mollusk {

    use mollusk_svm::program::keyed_account_for_system_program;
    use mollusk_svm::{Mollusk, program};
    use mollusk_svm::result::Check;
    use pinocchio::account_info::AccountInfo;
    use pinocchio::pubkey;
    use pinocchio::sysvars::rent::{self, Rent};
    use pvault::{InitializeAccounts, InitializeInstructionData, Vault, initialize};
    use solana_sdk::message::{AccountMeta, Instruction};
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::account::Account;

    const PROGRAM_ID: Pubkey = Pubkey::new_from_array(pvault::ID);
    const ALICE: Pubkey = Pubkey::new_from_array([9;32]);

    #[test]
    fn initialize() {
        
        // Loads the program to its cache
        let program_id = Pubkey::from(pvault::ID);
        let mollusk = Mollusk::new(&program_id, "pvault");
        let (system_program, _) = program::keyed_account_for_system_program();
        
        // Setup keys
        let vault_pda = 
        Pubkey::find_program_address( &[
                pvault::Vault::VAULT_SEED, 
                &ALICE.to_bytes(),
            ], &program_id);

        let vault_data_pda = 
        Pubkey::find_program_address( &[
                pvault::Vault::VAULT_DATA_SEED, 
                &ALICE.to_bytes(),
            ], &program_id);

        // Accounts (Provided in the mollusk call, this sets the account states when the test run.
        let accounts = &[
            (ALICE, Account::new(LAMPORTS_PER_SOL, 0, &system_program)),
            (vault_data_pda.0, Account::new(0, 0, &system_program)),
            (vault_pda.0, Account::new(0, 0, &system_program)),
            program::keyed_account_for_system_program(), // Adding the system_program to SVM
        ];

        // Create the instruction
        let inst_data = InitializeInstructionData::get_packed_instruction_data(false).to_vec();

        let inst = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(ALICE, true), 
                AccountMeta::new(vault_data_pda.0, false),
                AccountMeta::new(vault_pda.0, false),
                AccountMeta::new_readonly(system_program, false),
            ],
            data: inst_data,
        };

        mollusk.process_and_validate_instruction(
            &inst, 
            accounts,
        &[Check::success()]);
    }

    #[test]
    fn lock() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "pvault");
        let (system_program, system_account) = keyed_account_for_system_program();
        
        let vault_data_pda = Pubkey::find_program_address(&[Vault::VAULT_DATA_SEED, ALICE.as_ref()], &PROGRAM_ID);
        let vault_pda = Pubkey::find_program_address(&[Vault::VAULT_SEED, ALICE.as_ref()], &PROGRAM_ID);

        let accounts = &[
            (ALICE, Account::new(LAMPORTS_PER_SOL, 0, &system_program)),
            (vault_data_pda.0, Account::new(0, 0, &system_program)),
            (vault_pda.0, Account::new(0, 0, &system_program)),
            program::keyed_account_for_system_program(), // Adding the system_program to SVM
        ];




    }
}
