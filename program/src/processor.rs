use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::ChudexInstruction;

pub mod deposit;
pub mod exchange;
pub mod initialize_pool;
pub mod withdraw;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ChudexInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            ChudexInstruction::InitializePool { fee, fee_decimals } => {
                msg!("Instruction: InitializePool");
                initialize_pool::process(program_id, accounts, fee, fee_decimals)?;
            }
            ChudexInstruction::Deposit {
                pool_token_amount,
                max_token_a_amount,
                max_token_b_amount,
            } => {
                msg!("Instruction: Deposit");
                deposit::process(
                    program_id,
                    accounts,
                    pool_token_amount,
                    max_token_a_amount,
                    max_token_b_amount,
                )?;
            }
            ChudexInstruction::Withdraw {
                pool_token_amount,
                min_token_a_amount,
                min_token_b_amount,
            } => {
                msg!("Instruction: Withdraw");
                withdraw::process(
                    program_id,
                    accounts,
                    pool_token_amount,
                    min_token_a_amount,
                    min_token_b_amount,
                )?;
            }
            ChudexInstruction::Exchange {
                amount_in,
                min_amount_out,
            } => {
                msg!("Instruction: Withdraw");
                exchange::process(program_id, accounts, amount_in, min_amount_out)?;
            }
        }

        Ok(())
    }
}
