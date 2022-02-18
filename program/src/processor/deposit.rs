use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::{error::ChudexError, state::Pool};

use borsh::{BorshDeserialize, BorshSerialize};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_a_amount: u64,
    max_token_b_amount: u64,
) -> ProgramResult {

    // FETCH ACCOUNTS

    // ACCOUNT VALIDATION

    // LOGIC

    // calculate how much of each token to deposit

    // deposit

    // calculate how much pool token to mint

    // mint

    Ok(())
}
