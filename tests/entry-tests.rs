#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use mollusk_svm::*;
    use solana_sdk::account::Account;
    use solana_sdk::message::AccountMeta;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::pubkey;
    use pvault::*;


    const ID: Pubkey = pubkey!("pinzWHbUW7wQ16XcWcEjw4fucktuoHCnBfbbGB93PFo");
    const ALICE: Pubkey = Pubkey::new_from_array([0x01; 32]);
    const BOB: Pubkey = Pubkey::new_from_array([0x02; 32]);

    #[test]
    fn initialize() {
        // Pubkeys of all accounts needed for the transactions
        let (pda, bump) = Pubkey::find_program_address(&[b"vault"], &ID);
        let (system_program, system_account) = program::keyed_account_for_system_program();
        
        // Mollusk instance
        let mol: Mollusk = Mollusk::new(&ID, "../target/sbpf-solana-solana/release/pvault");
        
        // Build the accounts (At this point in time, the SVM is empty
        //      // Inject data to the accounts (if needed)
        let account = Account::new(LAMPORTS_PER_SOL, 0, &system_program);
        let state_account = Account::new(0,0, &system_program);
        
        // Get account meta
        let ix_accs = vec![
            AccountMeta::new(ALICE, true),
            AccountMeta::new(ALICE, true),

        ];

        // Data

        // Build IX
        
        // Get Tx Accounts

        // Process and validate the instruction
    }
}
