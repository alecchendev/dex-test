use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    msg,
    pubkey::Pubkey,
    program_pack::Pack,
    program::{invoke, invoke_signed},
    instruction::{AccountMeta},
};

use crate::{
    error::ChudexError,
    state::Pool,
    utils::{assert_msg, pubkey_cmp},
};

use borsh::{BorshDeserialize, BorshSerialize};

use spl_token::{
    instruction,
    state::{Account as TokenAccount, Mint}
};
use spl_associated_token_account::{
    id,
    create_associated_token_account,
};

use std::mem;

const FEE: u64 = 5;
const FEE_DECIMALS: u8 = 3;
const POOL_MINT_DECIMALS: u8 = 9;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u64,
    fee_decimals: u8,
) -> ProgramResult {

    // GET ACCOUNTS
    let accounts_iter = &mut accounts.iter();
    let user = next_account_info(accounts_iter)?;
    let pool_ai = next_account_info(accounts_iter)?;
    let pool_vault_a_ai = next_account_info(accounts_iter)?;
    let pool_vault_b_ai = next_account_info(accounts_iter)?;
    let mint_a_ai = next_account_info(accounts_iter)?;
    let mint_b_ai = next_account_info(accounts_iter)?;
    let pool_mint_ai = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let sysvar_rent = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    // deserialize accounts
    // let pool = Pool::try_from_slice(&pool_ai.try_borrow_data()?)?;
    // let pool_vault_a = TokenAccount::unpack(&pool_vault_a_ai.try_borrow_data()?)?;
    // let pool_vault_b = TokenAccount::unpack(&pool_vault_b_ai.try_borrow_data()?)?;
    // let mint_a = Mint::unpack(&mint_a_ai.try_borrow_data()?)?;
    // let mint_b = Mint::unpack(&mint_b_ai.try_borrow_data()?)?;
    // let pool_mint = Mint::unpack(&pool_mint_ai.try_borrow_data()?)?;

    // ACCOUNT VALIDATION

    // user is signer
    assert_msg(
        user.is_signer,
        ProgramError::MissingRequiredSignature,
        "User not signer",
    )?;
    
    // pool is owner of vaults and mint_auth of mint
    
    // token_program address
    assert_msg(
        *token_program.key == spl_token::id(),
        ChudexError::InvalidAccountAddress.into(),
        "Token program wrong address",
    )?;

    // check vaults match mints

    // pdas

    // pool
    let (mint_a_ai, mint_b_ai) = if pubkey_cmp(*mint_a_ai.key, *mint_b_ai.key) == 0 {
        (mint_a_ai, mint_b_ai)
    } else {
        (mint_b_ai, mint_a_ai)
    };
    msg!("mint_a_ai.key: {}", *mint_a_ai.key);
    msg!("mint_b_ai.key: {}", *mint_b_ai.key);
    let (pool_key, pool_bump) = Pubkey::find_program_address(
        &[ b"chudex_pool", mint_a_ai.key.as_ref(), mint_b_ai.key.as_ref() ],
        program_id
    );
    let pool_seeds = &[ b"chudex_pool", mint_a_ai.key.as_ref(), mint_b_ai.key.as_ref(), &[pool_bump] ];

    // assert_msg(
    //     *pool_ai.key == pool_key,
    //     ChudexError::InvalidProgramAddress.into(),
    //     "Pool address invalid",
    // )?;

    assert_msg(
        pool_ai.data_len() == 0,
        ChudexError::AccountAlreadyInitialized.into(),
        "Pool already initialized",
    )?;

    // vault a
    // vault b
    // mint

    // LOGIC

    // create token vaults (if not initialized)
    // create vault a
    let (vault_a_key, vault_a_bump) = Pubkey::find_program_address(
        &[ pool_ai.key.as_ref(), token_program.key.as_ref(), mint_a_ai.key.as_ref() ],
        &spl_associated_token_account::id(),
    );
    assert_msg(
        vault_a_key == *pool_vault_a_ai.key,
        ChudexError::InvalidProgramAddress.into(),
        "vault a pda aint right"
    )?;
    if pool_vault_a_ai.data_len() == 0 {
        msg!("initializing vault a...");
        msg!("pool_vault_a_ai.key: {}", *pool_vault_a_ai.owner);

        // create account
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                user.key,
                pool_ai.key,
                mint_a_ai.key,
            ),
            &[
                user.clone(),
                pool_vault_a_ai.clone(),
                pool_ai.clone(),
                mint_a_ai.clone(),
                system_program.clone(),
                token_program.clone(),
                sysvar_rent.clone(),
                associated_token_program.clone()
            ],
        );
        // invoke_signed(
        //     &system_instruction::create_account(
        //         user.key,
        //         pool_vault_a_ai.key,
        //         Rent::get()?.minimum_balance(TokenAccount::LEN),
        //         TokenAccount::LEN as u64,
        //         token_program.key,
        //     ),
        //     &[user.clone(), pool_vault_a_ai.clone(), system_program.clone()],
        //     &[&[ pool_ai.key.as_ref(), token_program.key.as_ref(), mint_a_ai.key.as_ref(), &[vault_a_bump] ]],
        //     // &[ pool_seeds ],
        // )?;

        // invoke_signed(
        //     &instruction::initialize_account(
        //         &spl_token::id(),
        //         pool_vault_a_ai.key,
        //         mint_a_ai.key,
        //         pool_ai.key
        //     )?,
        //     &[ pool_vault_a_ai.clone(), mint_a_ai.clone(), pool_ai.clone(), sysvar_rent.clone() ],
        //     &[ pool_seeds ],
        // )?;
        msg!("initialized vault a");
    }

    // create vault b
    if pool_vault_b_ai.data_len() == 0 {
        msg!("initializing vault b...");
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                user.key,
                pool_ai.key,
                mint_b_ai.key,
            ),
            &[
                user.clone(),
                pool_vault_b_ai.clone(),
                pool_ai.clone(),
                mint_b_ai.clone(),
                system_program.clone(),
                token_program.clone(),
                sysvar_rent.clone(),
                associated_token_program.clone()
            ],
        );
        msg!("initialized vault b");
    }

    // create mint
    if pool_mint_ai.data_len() == 0 {
        msg!("Initializing mint...");
        invoke_signed(
            &instruction::initialize_mint(
                &spl_token::id(),
                pool_mint_ai.key,
                pool_ai.key,
                Some(pool_ai.key),
                POOL_MINT_DECIMALS,
            )?,
            &[ pool_mint_ai.clone(), token_program.clone(), system_program.clone() ],
            &[ pool_seeds ],
        )?;
        msg!("Initialized mint");
    }

    // create pool account to store data
    msg!("initializing pool...");
    invoke_signed(
        &system_instruction::create_account(
            user.key,
            pool_ai.key,
            Rent::get()?.minimum_balance(mem::size_of::<Pool>()),
            mem::size_of::<Pool>() as u64,
            program_id,
        ),
        &[ user.clone(), pool_ai.clone(), system_program.clone()],
        &[ pool_seeds ],
    )?;

    // create pool struct and serialize data    
    let pool_vault_a = TokenAccount::unpack(&pool_vault_a_ai.try_borrow_data()?)?;
    let pool_vault_b = TokenAccount::unpack(&pool_vault_b_ai.try_borrow_data()?)?;
    let vault_a = if pool_vault_a.mint == *mint_a_ai.key { *pool_vault_a_ai.key } else { *pool_vault_b_ai.key };
    let vault_b = if pool_vault_a.mint == *mint_a_ai.key { *pool_vault_b_ai.key } else { *pool_vault_a_ai.key };
    let pool = Pool {
        vault_a: vault_a,
        vault_b: vault_b,
        mint: *pool_mint_ai.key,
        fee: FEE,
        fee_decimals: FEE_DECIMALS,
    };
    pool.serialize(&mut *pool_ai.try_borrow_mut_data()?)?;
    msg!("initialized pool");

    Ok(())
}
