use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Receipt {
    pub withdrawal_id: u64,
    pub recipient: Pubkey,
    pub amount: u64,
    pub journal_seq: u64,
    pub slot: u64,
}
