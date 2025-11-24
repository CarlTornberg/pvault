// #![cfg(feature = "test-sbf")]

#[allow(unused_imports)]
#[cfg(test)]
mod mollusk {

    use mollusk_svm::Mollusk;
    use pvault::{InitializeAccounts, InitializeData};
    use solana_sdk::message::Instruction;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::account::Account;

    #[test]
    fn initialize() {
        // Provided accounts
        
        // Buffer
        
        // Sysvars
        
        // Loads the program to its cache
        let mollusk = Mollusk::new(&Pubkey::from(pvault::ID), "pvault");
        
        // Setup keys
        let alice = Pubkey::new_unique();

        // Accounts (Provided in the mollusk call, this sets the account states when the test run.
        let accounts = &[
            Account::new(1_000_000, 0, &alice)
        ];

        let init_data = InitializeData { locked: &false };
        let inst = Instruction {
            program_id:  Pubkey::from(pvault::ID),
            accounts: vec![],
            data: vec![],
        };
    }
}
