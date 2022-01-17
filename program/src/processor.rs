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
            ChudexInstruction::InitializePool { } => {
                msg!("Instruction: InitializeChudex");
                initialize_pool::process(program_id, accounts)?;
            }
            ChudexInstruction::Deposit { } => {
                msg!("Instruction: Deposit");
                deposit::process(program_id, accounts)?;
            }
            ChudexInstruction::Withdraw { } => {
                msg!("Instruction: Withdraw");
                withdraw::process(program_id, accounts)?;
            }
            ChudexInstruction::Exchange { } => {
                msg!("Instruction: Withdraw");
                exchange::process(program_id, accounts)?;
            }
        }

        Ok(())
    }
}
