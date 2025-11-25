// #![cfg(feature = "test-sbf")]

#[allow(unused_imports)]
#[cfg(test)]
mod mollusk {

    use mollusk_svm::{Mollusk, program};
    use mollusk_svm::result::Check;
    use pinocchio::account_info::AccountInfo;
    use pinocchio::pubkey;
    use pinocchio::sysvars::rent::{self, Rent};
    use pvault::{InitializeAccounts, InitializeInstructionData};
    use solana_sdk::message::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::account::Account;

    #[test]
    fn initialize() {
        
        // Loads the program to its cache
        let program_id = Pubkey::from(pvault::ID);
        let mollusk = Mollusk::new(&program_id, "pvault");
        let (system_program, system_account) = program::keyed_account_for_system_program();
        
        // Setup keys
        let alice = Pubkey::new_unique();
        let vault_pda = 
        Pubkey::find_program_address(
            &[
                pvault::Vault::VAULT_SEED, 
                &alice.to_bytes(),
            ], 
            &program_id);
        let vault_data_pda = 
        Pubkey::find_program_address(
            &[
                pvault::Vault::VAULT_DATA_SEED, 
                &alice.to_bytes(),
            ], 
            &program_id);

        // Accounts (Provided in the mollusk call, this sets the account states when the test run.
        let accounts = &[
            (alice, Account::new(1_000_000, 0, &system_program)),
            (vault_data_pda.0, Account::new(0, 0, &system_program)),
            (vault_pda.0, Account::new(0, 0, &system_program)),
            program::keyed_account_for_system_program(), // Adding the system_program to SVM
        ];

        // Create the instruction
        let init_data = InitializeInstructionData { locked: &false };
        let inst = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(alice, true), 
                AccountMeta::new(vault_data_pda.0, true),
                AccountMeta::new(vault_pda.0, true),
                AccountMeta::new_readonly(system_program, false),
            ],
            data: init_data.pack().to_vec(),
        };

        mollusk.process_and_validate_instruction(
            &inst, 
            accounts,
        &[Check::success()]);
    }
}
