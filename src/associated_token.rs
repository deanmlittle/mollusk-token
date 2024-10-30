use mollusk_svm::Mollusk;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkey;

use crate::elf;

pub const ID: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

pub fn add_program(mollusk: &mut Mollusk) {
    mollusk.add_program_with_elf_and_loader(&ID, &include_bytes!("associated_token.so"), loader_key);
}

/// Get the key and account for the system program.
pub fn keyed_account() -> (Pubkey, AccountSharedData) {
    (
        ID,
        mollusk_svm::program::create_program_account_loader_v3(&ID)
    )
}