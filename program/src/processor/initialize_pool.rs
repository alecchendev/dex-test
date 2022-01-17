use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey
};

use crate::{
    error::ChudexError,
    state::Pool,
};

use borsh::{BorshDeserialize, BorshSerialize};


pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u64,
    fee_decimals: u8,
) -> ProgramResult {
    Ok(())
}
