use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    system_program::id as system_program_id,
    sysvar::{rent, Sysvar},
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
    let user_token_a_ai = next_account_info(accounts_iter)?;
    let user_token_b_ai = next_account_info(accounts_iter)?;
    let user_pool_token = next_account_info(accounts_iter)?;
    let pool_ai = next_account_info(accounts_iter)?;
    let pool_vault_a_ai = next_account_info(accounts_iter)?;
    let pool_vault_b_ai = next_account_info(accounts_iter)?;
    let pool_mint = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let sysvar_rent = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    // deserialization
    let pool = Pool::try_from_slice(&pool_ai.try_borrow_data()?)?;

    // ACCOUNT VALIDATION

    // token account ownership
    // user token accounts
    // pool vault accounts
    // pool is mint authority

    // pda verification
    let (pool_key, pool_bump) = Pubkey::find_program_address(
        &[
            b"chudex_pool",
            // mint_a_ai.key.as_ref(),
            // mint_b_ai.key.as_ref(),
            pool.vault_a.as_ref(),
            pool.vault_b.as_ref(),
        ],
        program_id,
    );
    let pool_seeds = &[
        b"chudex_pool",
        // mint_a_ai.key.as_ref(),
        // mint_b_ai.key.as_ref(),
        pool.vault_a.as_ref(),
        pool.vault_b.as_ref(),
        &[pool_bump],
    ];

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
    let pool_vault_a = TokenAccount::unpack_from_slice(&pool_vault_a_ai.try_borrow_data()?)?;
    let pool_vault_b = TokenAccount::unpack_from_slice(&pool_vault_b_ai.try_borrow_data()?)?;
    let token_b_amount = if pool_vault_a.amount == 0 || pool_vault_b.amount == 0 {
        max_token_b_amount
    } else {
        ((token_a_amount as f64) * (pool_vault_b.amount as f64) / (pool_vault_a.amount as f64)) as u64
    };
    msg!("Got token amounts");

    // deposit
    // deposit token 1
    invoke(
        &instruction::transfer(
            &spl_token::id(),
            &user_token_a_ai.key,
            &pool_vault_a_ai.key,
            &user.key,
            &[&user.key],
            token_a_amount)?,
        &[user_token_a_ai.clone(), pool_vault_a_ai.clone(), user.clone()],
    )?;

    // deposit token 2
    invoke(
        &instruction::transfer(
            &spl_token::id(),
            &user_token_b_ai.key,
            &pool_vault_b_ai.key,
            &user.key,
            &[&user.key],
            token_b_amount)?,
        &[user_token_b_ai.clone(), pool_vault_b_ai.clone(), user.clone()],
    )?;

    // calculate how much pool token to mint
    // - greater decimal token amount, tie broken by vault_a before vault_b
    let pool_token_amount = if pool.vault_a == *pool_vault_a_ai.key {
        token_a_amount
    } else {
        token_b_amount
    };
    msg!("Got pool token amount");

    // initialize pool token user account if needed
    if user_pool_token.data_len() == 0 {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                user.key,
                user.key,
                pool_mint.key,
            ),
            &[
                user.clone(),
                user_pool_token.clone(),
                user.clone(),
                pool_mint.clone(),
                system_program.clone(),
                token_program.clone(),
                sysvar_rent.clone(),
                associated_token_program.clone(),
            ],
        )?;
    }

    // mint to user
    invoke_signed(
        &instruction::mint_to(
            &spl_token::id(),
            &pool_mint.key,
            &user_pool_token.key,
            &pool_ai.key,
            &[&pool_ai.key],
            pool_token_amount)?,
        &[ pool_mint.clone(), user_pool_token.clone(), pool_ai.clone() ],
        &[pool_seeds],
    )?;

    Ok(())
}
