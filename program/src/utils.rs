use solana_program::{
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
};

pub fn assert_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

pub fn pubkey_cmp(a: Pubkey, b: Pubkey) -> u8 {
    if a < b {
        0
    } else {
        1
    }
}