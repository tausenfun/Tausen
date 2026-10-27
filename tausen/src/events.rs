use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub usdc_mint: Pubkey,
    pub vault: Pubkey,
}

#[event]
pub struct Deposited {
    pub user: Pubkey,
    pub amount: u64,
    pub vault_balance: u64,
}

#[event]
pub struct Withdrawn {
    pub withdrawal_id: u64,
    pub recipient: Pubkey,
    pub amount: u64,
    pub journal_seq: u64,
    pub vault_balance: u64,
}

#[event]
pub struct Recorded {
    pub seq: u64,
    pub kind: u8,
    pub wallet: Pubkey,
    pub token: Pubkey,
    pub amount: i64,
    pub data_hash: [u8; 32],
    pub at: i64,
    pub chain: [u8; 32],
}

#[event]
pub struct OperatorChanged {
    pub previous: Pubkey,
    pub operator: Pubkey,
}

#[event]
pub struct PausedChanged {
    pub paused: bool,
}

#[event]
pub struct AdminProposed {
    pub next: Pubkey,
}

#[event]
pub struct AdminChanged {
    pub previous: Pubkey,
    pub admin: Pubkey,
}
