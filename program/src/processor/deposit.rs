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
    error::TokenError,
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
    let pool_mint_ai = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let sysvar_rent = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    // deserialization
    let pool = Pool::try_from_slice(&pool_ai.try_borrow_data()?)?;
    let user_token_a = TokenAccount::unpack_from_slice(&user_token_a_ai.try_borrow_data()?)?;
    let user_token_b = TokenAccount::unpack_from_slice(&user_token_b_ai.try_borrow_data()?)?;
    let pool_vault_a = TokenAccount::unpack_from_slice(&pool_vault_a_ai.try_borrow_data()?)?;
    let pool_vault_b = TokenAccount::unpack_from_slice(&pool_vault_b_ai.try_borrow_data()?)?;
    let pool_mint = Mint::unpack_from_slice(&pool_mint_ai.try_borrow_data()?)?;

    // ACCOUNT VALIDATION

    // token account ownership
    // user token accounts
    assert_msg(
        user_token_a.owner == *user.key,
        TokenError::OwnerMismatch.into(),
        "user token a not owned by user",
    )?;
    assert_msg(
        user_token_b.owner == *user.key,
        TokenError::OwnerMismatch.into(),
        "user token b not owned by user",
    )?;

    // pool vault accounts
    assert_msg(
        pool_vault_a.owner == *pool_ai.key,
        TokenError::OwnerMismatch.into(),
        "pool vault a not owned by pool",
    )?;
    assert_msg(
        pool_vault_b.owner == *pool_ai.key,
        TokenError::OwnerMismatch.into(),
        "pool vault b not owned by pool",
    )?;

    // pool is mint authority
    assert_msg(
        pool_mint.mint_authority.unwrap() == *pool_ai.key,
        TokenError::InvalidMint.into(),
        "pool not mint authority of pool mint",
    )?;

    // pda verification

    // user token a pda
    let (user_token_a_key, _) = Pubkey::find_program_address(
        &[
            user.key.as_ref(),
            token_program.key.as_ref(),
            pool_vault_a.mint.as_ref(),
        ],
        &spl_associated_token_account::id(),
    );
    assert_msg(
        user_token_a_key == *user_token_a_ai.key,
        ChudexError::InvalidProgramAddress.into(),
        "user token account a pda aint right",
    )?;

    // user token b pda
    let (user_token_b_key, _) = Pubkey::find_program_address(
        &[
            user.key.as_ref(),
            token_program.key.as_ref(),
            pool_vault_b.mint.as_ref(),
        ],
        &spl_associated_token_account::id(),
    );
    assert_msg(
        user_token_b_key == *user_token_b_ai.key,
        ChudexError::InvalidProgramAddress.into(),
        "user token account b pda aint right",
    )?;

    // user pool token pda
    let (user_pool_token_key, _) = Pubkey::find_program_address(
        &[
            user.key.as_ref(),
            token_program.key.as_ref(),
            pool_mint_ai.key.as_ref(),
        ],
        &spl_associated_token_account::id(),
    );
    assert_msg(
        user_pool_token_key == *user_pool_token.key,
        ChudexError::InvalidProgramAddress.into(),
        "user pool token account pda aint right",
    )?;

    // vault a pda
    let (vault_a_key, vault_a_bump) = Pubkey::find_program_address(
        &[
            pool_ai.key.as_ref(),
            token_program.key.as_ref(),
            pool_vault_a.mint.as_ref(),
        ],
        &spl_associated_token_account::id(),
    );
    assert_msg(
        vault_a_key == *pool_vault_a_ai.key,
        ChudexError::InvalidProgramAddress.into(),
        "vault a pda aint right",
    )?;

    // vault b pda
    let (vault_b_key, vault_b_bump) = Pubkey::find_program_address(
        &[
            pool_ai.key.as_ref(),
            token_program.key.as_ref(),
            pool_vault_b.mint.as_ref(),
        ],
        &spl_associated_token_account::id(),
    );
    assert_msg(
        vault_b_key == *pool_vault_b_ai.key,
        ChudexError::InvalidProgramAddress.into(),
        "vault b pda aint right",
    )?;

    // pool pda
    let (pool_key, pool_bump) = Pubkey::find_program_address(
        &[
            b"chudex_pool",
            // mint_a_ai.key.as_ref(),
            // mint_b_ai.key.as_ref(),
            pool.mint_a.as_ref(),
            pool.mint_b.as_ref(),
        ],
        program_id,
    );
    let pool_seeds = &[
        b"chudex_pool",
        // mint_a_ai.key.as_ref(),
        // mint_b_ai.key.as_ref(),
        pool.mint_a.as_ref(),
        pool.mint_b.as_ref(),
        &[pool_bump],
    ];
    assert_msg(
        *pool_ai.key == pool_key,
        ChudexError::InvalidAccountAddress.into(),
        "Pool address invalid",
    )?;

    // pool mint pda
    let (pool_mint_key, pool_mint_bump) =
        Pubkey::find_program_address(&[b"chudex_pool_mint", pool_ai.key.as_ref()], program_id);
    let pool_mint_seeds = &[b"chudex_pool_mint", pool_ai.key.as_ref(), &[pool_mint_bump]];
    assert_msg(
        *pool_mint_ai.key == pool_mint_key,
        ChudexError::InvalidAccountAddress.into(),
        "Pool mint address invalid",
    )?;

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
    let token_b_amount = if pool_vault_a.amount == 0 || pool_vault_b.amount == 0 {
        max_token_b_amount
    } else {
        ((token_a_amount as f64) * (pool_vault_b.amount as f64) / (pool_vault_a.amount as f64))
            as u64
    };

    if token_b_amount > max_token_b_amount {
        return Err(ChudexError::DepositAmountExceedsLimit.into());
    }

    msg!(
        "Got token amounts - a: {} b: {}",
        token_a_amount,
        token_b_amount
    );

    // deposit
    // deposit token 1
    invoke(
        &instruction::transfer(
            &spl_token::id(),
            &user_token_a_ai.key,
            &pool_vault_a_ai.key,
            &user.key,
            &[&user.key],
            token_a_amount,
        )?,
        &[
            user_token_a_ai.clone(),
            pool_vault_a_ai.clone(),
            user.clone(),
        ],
    )?;

    // deposit token 2
    invoke(
        &instruction::transfer(
            &spl_token::id(),
            &user_token_b_ai.key,
            &pool_vault_b_ai.key,
            &user.key,
            &[&user.key],
            token_b_amount,
        )?,
        &[
            user_token_b_ai.clone(),
            pool_vault_b_ai.clone(),
            user.clone(),
        ],
    )?;

    // calculate how much pool token to mint
    // - greater decimal token amount, tie broken by vault_a before vault_b
    msg!(
        "pool.mint_a: {} pool_vault_a.mint: {}",
        pool.mint_a,
        pool_vault_a.mint
    );
    msg!("pool.mint_b: {}", pool.mint_b);
    let pool_token_amount = if pool.mint_a == pool_vault_a.mint {
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
                pool_mint_ai.key,
            ),
            &[
                user.clone(),
                user_pool_token.clone(),
                user.clone(),
                pool_mint_ai.clone(),
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
            &pool_mint_ai.key,
            &user_pool_token.key,
            &pool_ai.key,
            &[&pool_ai.key],
            pool_token_amount,
        )?,
        &[
            pool_mint_ai.clone(),
            user_pool_token.clone(),
            pool_ai.clone(),
        ],
        &[pool_seeds],
    )?;

    Ok(())
}
