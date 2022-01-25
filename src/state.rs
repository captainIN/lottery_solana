use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Manager {
    pub address: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LotteryBox {
    pub participants: [Pubkey; 8],
    pub pool_size: f64,
    pub base_fee: f64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WinnerList {
    pub winners: Vec<Pubkey>,
}