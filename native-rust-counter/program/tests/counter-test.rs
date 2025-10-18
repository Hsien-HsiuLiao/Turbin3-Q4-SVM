#[cfg(test)]

mod tests {
  use {
    mollusk_svm::{program, Mollusk},
    solana_sdk::{
      account::Account, 
      instruction::{AccountMeta, Instruction}, 
      native_token::LAMPORTS_PER_SOL,
      pubkey::Pubkey,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::pubkey,
  };

  // Import the counter program types
  use counter_solana_native::state::Counter;

  const PROGRAM_ID: pubkey::Pubkey = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
  const COUNTER_ACCOUNT: pubkey::Pubkey = pubkey::Pubkey::new_from_array([0x02; 32]);

  #[test]
  fn test_increment_counter() {
    // Create mollusk instance
    let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/counter_solana_native");
   
    // Create system program account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Create counter account with initial state
    let initial_counter = Counter { count: 0 };
    let counter_data = initial_counter.try_to_vec().unwrap();
    let counter_account = Account::new(
      1 * LAMPORTS_PER_SOL, 
      counter_data.len(), 
      &PROGRAM_ID
    );

    // Create instruction accounts
    let ix_accounts: Vec<AccountMeta> = vec![
      AccountMeta::new(COUNTER_ACCOUNT, false), // Counter account (writable)
    ];

    // Create instruction data (discriminant 0 for increment)
    let instruction_data = vec![0u8]; // 0 = increment instruction

    // Build instruction
    let instruction = Instruction::new_with_bytes(
      PROGRAM_ID,
      &instruction_data,
      ix_accounts,
    );

    // Create transaction accounts
    let tx_accounts = vec![
      (COUNTER_ACCOUNT, counter_account),
      (system_program, system_account),
    ];

    // Process the instruction
    let result = mollusk.process_instruction(&instruction, &tx_accounts);
    
    // Verify the instruction succeeded
    assert!(result.program_result.is_ok(), "Instruction should succeed");
    
    // Verify the counter was incremented
    let updated_counter_account = result.resulting_accounts
      .iter()
      .find(|(pubkey, _)| *pubkey == COUNTER_ACCOUNT)
      .map(|(_, account)| account)
      .expect("Counter account should exist");
    
    let updated_counter = Counter::try_from_slice(&updated_counter_account.data)
      .expect("Should deserialize counter");
    
    assert_eq!(updated_counter.count, 1, "Counter should be incremented to 1");
  }

  






}