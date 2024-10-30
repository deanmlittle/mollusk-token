use mollusk_svm::Mollusk;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkey;

pub const ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub fn add_program(mollusk: &mut Mollusk) {
    mollusk.add_program_with_elf_and_loader(&ID, include_bytes!("elf/token.so"), loader_key);
}

pub fn account() -> AccountSharedData {
    mollusk_svm::program::create_program_account_loader_v3(&ID)
}

/// Get the key and account for the system program.
pub fn keyed_account() -> (Pubkey, AccountSharedData) {
    (
        ID,
        account()
    )
}