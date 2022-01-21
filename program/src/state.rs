use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::mem::size_of;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Pool {
    // TODO
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub mint: Pubkey,
    pub fee: u64,
    pub fee_decimals: u8,
}
