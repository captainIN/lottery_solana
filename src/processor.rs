use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::LotteryInstruction;
use crate::state::{Manager, LotteryBox, WinnerList};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = LotteryInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            LotteryInstruction::Start => {
                msg!("Instruction: Start");
                let accounts_iter = &mut accounts.iter();
                let manager_ai = next_account_info(accounts_iter)?;
                let mut manager = Manager::try_from_slice(&manager_ai.data.borrow())?;
                manager.address = *manager_ai.owner;
                msg!("manager_ai = {:#?}", *manager_ai);
                msg!("manager = {:#?}", manager);
                msg!("Updating Manager Address {}", manager.address);
                manager.serialize(&mut *manager_ai.data.borrow_mut())?;

                let lottery_ai = next_account_info(accounts_iter)?;
                let mut lottery_box = LotteryBox::try_from_slice(&lottery_ai.data.borrow())?;
                lottery_box.participants = [*lottery_ai.owner; 8];
                lottery_box.pool_size = 0.0;
                lottery_box.base_fee = 1.0;
                msg!("lottery_box = {:#?}", lottery_box);
                lottery_box.serialize(&mut *lottery_ai.data.borrow_mut())?;
            },
            LotteryInstruction::Participate => {
                msg!("Instruction: Participate");
                let accounts_iter = &mut accounts.iter();
                let lottery_ai = next_account_info(accounts_iter)?;
                let mut lottery_box = LotteryBox::try_from_slice(&lottery_ai.data.borrow())?;
                msg!("lottery_box before = {:#?}", lottery_box);
                // lottery_box.participants[1] = *lottery_ai.owner;
                lottery_box.pool_size = 0.0;
                lottery_box.base_fee = 1.0;
                // lottery_box.participants.push(*lottery_ai.owner);
                // lottery_box.pool_size += lottery_box.base_fee;
                msg!("lottery_ai = {:#?}", *lottery_ai);
                msg!("lottery_box after = {:#?}", lottery_box);
                lottery_box.serialize(&mut *lottery_ai.data.borrow_mut())?;
            },
            LotteryInstruction::Result => {
                msg!("Instruction: Result");
            }
        }
        Ok(())
    }
}
