use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ChudexInstruction {
    InitializePool {
        // TODO
     },
    Deposit {
        // TODO
    },
    Withdraw {
        // TODO
    },
    Exchange {
        // TODO
    },
}
