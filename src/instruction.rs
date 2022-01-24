use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum LotteryInstruction {
    Start, // unsigned byte
    Participate, // unsigned byte
    Result, // unsigned byte
}
