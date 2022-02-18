use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ChudexInstruction {
    /// Initializes a new pool. Creates mint and accounts for pool struct and token vaults.
    ///
    /// Accounts:
    /// [signer] user
    /// [writable] pool
    /// [writable] pool token acc a
    /// [writable] pool token acc b
    /// mint a
    /// mint b
    /// [writable] pool token mint
    /// token program
    /// system program
    /// sysvar
    /// associated token program
    InitializePool {
        // TODO
        fee: u64,
        fee_decimals: u8,
    },

    /// Provides liquidity at current exchange rate for both tokens.
    /// Mints pool tokens to user. Creates user pool token acc if needed.
    ///
    /// Accounts:
    /// [signer, writable] user
    /// [writable] user token acc a
    /// [writable] user token acc b
    /// [writable] user pool token acc
    /// pool
    /// [writable] pool token acc a
    /// [writable] pool token acc b
    /// [wriatble] pool token mint
    /// token program
    /// system program
    /// sysvar
    /// associated token program
    Deposit {
        // TODO
        token_a_amount: u64,
        max_token_b_amount: u64,
    },

    /// Withdraws tokens from pool at current exchange rate.
    /// Burns pool tokens.
    ///
    /// Accounts:
    /// user
    /// user token acc a
    /// user token acc b
    /// user pool token acc
    /// pool
    /// pool token acc a
    /// pool token acc b
    /// pool token mint
    /// token program
    Withdraw {
        // TODO
        pool_token_amount: u64,
        min_token_a_amount: u64,
        min_token_b_amount: u64,
    },

    /// Swaps one token for another.
    ///
    /// Accounts:
    /// user
    /// user token src acc
    /// user token dst acc
    /// pool
    /// pool token src acc
    /// pool token dst acc
    /// token program
    Exchange {
        // TODO
        amount_in: u64,
        min_amount_out: u64,
    },
}
