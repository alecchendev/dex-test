use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::AccountMeta,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction, system_program as system_program_ext,
    sysvar::{rent, Sysvar},
};

use bs58;
use std::cmp;

use crate::{error::ChudexError, state::Pool, utils::assert_msg};

use borsh::{BorshDeserialize, BorshSerialize};

use spl_associated_token_account::{create_associated_token_account, id};
use spl_token::{
    instruction,
    state::{Account as TokenAccount, Mint},
};

use std::mem;

const FEE: u64 = 5;
const FEE_DECIMALS: u8 = 3;
const POOL_MINT_DECIMALS: u8 = 9;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u64,
    fee_decimals: u64,
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

    // ACCOUNT VALIDATION

    // user is signer
    assert_msg(
        user.is_signer,
        ProgramError::MissingRequiredSignature,
        "User not signer",
    )?;

    // PDAs

    // vault a pda
    let (vault_a_key, vault_a_bump) = Pubkey::find_program_address(
        &[
            pool_ai.key.as_ref(),
            token_program.key.as_ref(),
            mint_a_ai.key.as_ref(),
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
            mint_b_ai.key.as_ref(),
        ],
        &spl_associated_token_account::id(),
    );
    assert_msg(
        vault_b_key == *pool_vault_b_ai.key,
        ChudexError::InvalidProgramAddress.into(),
        "vault b pda aint right",
    )?;

    // pool pda
    let mint_a = Mint::unpack_from_slice(&mint_a_ai.try_borrow_data()?)?;
    let mint_b = Mint::unpack_from_slice(&mint_b_ai.try_borrow_data()?)?;
    let (mint_a_seed, mint_b_seed) = if mint_a.decimals == mint_b.decimals {
        if bs58::encode(mint_a_ai.key).into_string() < bs58::encode(mint_b_ai.key).into_string() {
            (mint_a_ai.key, mint_b_ai.key)
        } else {
            (mint_b_ai.key, mint_a_ai.key)
        }
    } else {
        if mint_a.decimals > mint_b.decimals {
            (mint_a_ai.key, mint_b_ai.key)
        } else {
            (mint_b_ai.key, mint_a_ai.key)
        }
    };
    msg!(
        "mint a pubkey: {}\nmint b pubkey: {}\nmint a seed: {}\nmint b seed: {}",
        mint_a_ai.key.to_string(),
        mint_b_ai.key.to_string(),
        mint_a_seed,
        mint_b_seed.to_string()
    );
    let (pool_key, pool_bump) = Pubkey::find_program_address(
        &[
            b"chudex_pool",
            // mint_a_ai.key.as_ref(),
            // mint_b_ai.key.as_ref(),
            mint_a_seed.as_ref(),
            mint_b_seed.as_ref(),
        ],
        program_id,
    );
    let pool_seeds = &[
        b"chudex_pool",
        // mint_a_ai.key.as_ref(),
        // mint_b_ai.key.as_ref(),
        mint_a_seed.as_ref(),
        mint_b_seed.as_ref(),
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
        *system_program.key == system_program_ext::id(),
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

    // check data not initialized
    // vault a data
    assert_msg(
        pool_vault_a_ai.data_len() == 0,
        ChudexError::AccountAlreadyInitialized.into(),
        "Vault a already initialized",
    )?;

    // vault b data
    assert_msg(
        pool_vault_b_ai.data_len() == 0,
        ChudexError::AccountAlreadyInitialized.into(),
        "Vault b already initialized",
    )?;

    // pool data
    assert_msg(
        pool_ai.data_len() == 0,
        ChudexError::AccountAlreadyInitialized.into(),
        "Pool already initialized",
    )?;

    // LOGIC

    // create token vaults
    // create vault a
    msg!("initializing vault a...");

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
            associated_token_program.clone(),
        ],
    )?;
    msg!("initialized vault a");

    // create vault b
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
            associated_token_program.clone(),
        ],
    )?;
    msg!("initialized vault b");

    // create mint
    msg!("Initializing mint...");
    let mint_data_len = 82;
    invoke_signed(
        &system_instruction::create_account(
            user.key,
            pool_mint_ai.key,
            rent::Rent::get()?.minimum_balance(mint_data_len),
            mint_data_len as u64,
            &spl_token::id(),
        ),
        &[user.clone(), pool_mint_ai.clone(), system_program.clone()],
        &[pool_mint_seeds],
    )?;

    // find the larger decimal of the two token mints
    let pool_mint_decimals = cmp::max(mint_a.decimals, mint_b.decimals);
    invoke_signed(
        &instruction::initialize_mint(
            &spl_token::id(),
            pool_mint_ai.key,
            pool_ai.key,
            Some(pool_ai.key),
            pool_mint_decimals,
        )?,
        &[
            pool_mint_ai.clone(),
            sysvar_rent.clone(),
            token_program.clone(),
        ],
        &[pool_mint_seeds],
    )?;
    msg!("Initialized mint");

    // create pool account to store data
    msg!("initializing pool...");
    invoke_signed(
        &system_instruction::create_account(
            user.key,
            pool_ai.key,
            rent::Rent::get()?.minimum_balance(mem::size_of::<Pool>()),
            mem::size_of::<Pool>() as u64,
            program_id,
        ),
        &[user.clone(), pool_ai.clone(), system_program.clone()],
        &[pool_seeds],
    )?;

    // create pool struct and serialize data
    let pool = Pool {
        mint_a: *mint_a_seed,
        mint_b: *mint_b_seed,
        mint: *pool_mint_ai.key,
        fee: fee,
        fee_decimals: fee_decimals,
    };
    pool.serialize(&mut *pool_ai.try_borrow_mut_data()?)?;
    msg!("initialized pool");

    Ok(())
}
