use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub pending_admin: Option<Pubkey>,
    pub operator: Pubkey,
    pub usdc_mint: Pubkey,
    pub vault: Pubkey,
    pub last_record: u64,
    pub records: u64,
    pub journal_hash: [u8; 32],
    pub deposited: u64,
    pub withdrawn: u64,
    pub paused: bool,
    pub bump: u8,
    pub vault_bump: u8,
}
