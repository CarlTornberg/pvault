# pvault
Nocopy lightweight Pinocchio based vault

# Create project
```bash
cargo new pinocchio-vault --lib --edition 2021
cd pinocchio-vault
cargo add pinocchio pinocchio-system pinocchio-log pinocchio-pubkey
```

# Update Cargo.toml
```toml
[lib]
crate-type = ["lib", "cdylib"]
```

# Boilerplate in lib.rs
```rust
#![no_std]
use pinocchio::{
  account_info::AccountInfo,
  entrypoint,
  msg,
  ProgramResult,
  pubkey::Pubkey
};
use pinocchio_pubkey::declare_id;

entrypoint!(process_instruction);
declare_id!("YOUR_PROGRAM_PUBKEY");

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
    let (inst, data @ ..) = instruction_data else {
        return Err(....);
    }
    match inst {
        _ => msg!("Hello from my program!");
    }
  Ok(())
}
```
