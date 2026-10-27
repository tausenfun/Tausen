use anchor_lang::{prelude::*, solana_program::hash::hashv};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct Entry {
    pub seq: u64,
    pub kind: u8,
    pub wallet: Pubkey,
    pub token: Pubkey,
    pub amount: i64,
    pub data_hash: [u8; 32],
    pub at: i64,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    Deposit = 0,
    Withdraw = 1,
    Open = 2,
    Close = 3,
    Liquidation = 4,
    Payout = 5,
    PoolOpen = 6,
    PoolFund = 7,
    PoolRefund = 8,
    PoolExpire = 9,
}

impl EntryKind {
    pub const COUNT: u8 = 10;
}

impl Entry {
    pub fn chain(&self, previous: &[u8; 32]) -> [u8; 32] {
        hashv(&[
            previous,
            &self.seq.to_le_bytes(),
            &[self.kind],
            self.wallet.as_ref(),
            self.token.as_ref(),
            &self.amount.to_le_bytes(),
            &self.data_hash,
            &self.at.to_le_bytes(),
        ])
        .to_bytes()
    }
}
