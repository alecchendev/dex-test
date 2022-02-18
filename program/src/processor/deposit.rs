use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent, Sysvar},
    system_instruction, system_program::id as system_program_id,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{error::ChudexError, state::Pool, utils::assert_msg};

use spl_associated_token_account::{create_associated_token_account, id};
use spl_token::{
    instruction,
    state::{Account as TokenAccount, Mint},
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_a_amount: u64,
    max_token_b_amount: u64,
) -> ProgramResult {
    // FETCH ACCOUNTS
    let accounts_iter = &mut accounts.iter();

    let user = next_account_info(accounts_iter)?;
    let user_token_a = next_account_info(accounts_iter)?;
    let user_token_b = next_account_info(accounts_iter)?;
    let user_pool_token = next_account_info(accounts_iter)?;
    let pool_ai = next_account_info(accounts_iter)?;
    let pool_vault_a = next_account_info(accounts_iter)?;
    let pool_vault_b = next_account_info(accounts_iter)?;
    let pool_mint = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let sysvar_rent = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    // ACCOUNT VALIDATION

    // token account ownership
    // user token accounts
    // pool vault accounts
    // pool is mint authority

    // pda verification

    // external program verification
    // token program
    assert_msg(
        *token_program.key == spl_token::id(),
        ChudexError::InvalidAccountAddress.into(),
        "Token program wrong address",
    )?;

    // system program
    assert_msg(
        *system_program.key == system_program_id(),
        ChudexError::InvalidAccountAddress.into(),
        "System program wrong address",
    )?;

    // sysvar program
    assert_msg(
        *sysvar_rent.key == rent::id(),
        ChudexError::InvalidAccountAddress.into(),
        "Sysvar program wrong address",
    )?;

    // associated token program
    assert_msg(
        *associated_token_program.key == spl_associated_token_account::id(),
        ChudexError::InvalidAccountAddress.into(),
        "Associated token program wrong address",
    )?;

    // LOGIC

    // calculate how much of each token to deposit

    // deposit

    // calculate how much pool token to mint

    // mint

    Ok(())
}
