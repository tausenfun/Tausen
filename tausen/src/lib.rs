use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8YSZ78x9iPVRCMhz1vYTSkF4Ka1Jkv6K9349ypFijLc9");

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "Tausen",
    project_url: "https://tausen.fun",
    contacts: "link:https://x.com/TausenFun",
    policy: "Report vulnerabilities privately through https://x.com/TausenFun before disclosing them.",
    preferred_languages: "en"
}

#[program]
pub mod tausen {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, operator: Pubkey) -> Result<()> {
        instructions::initialize::process_initialize(ctx, operator)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::process_deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, withdrawal_id: u64, amount: u64, journal_seq: u64) -> Result<()> {
        instructions::withdraw::process_withdraw(ctx, withdrawal_id, amount, journal_seq)
    }

    pub fn record(ctx: Context<Record>, entry: Entry) -> Result<()> {
        instructions::record::process_record(ctx, entry)
    }

    pub fn set_operator(ctx: Context<Admin>, operator: Pubkey) -> Result<()> {
        instructions::admin::process_set_operator(ctx, operator)
    }

    pub fn set_paused(ctx: Context<Admin>, paused: bool) -> Result<()> {
        instructions::admin::process_set_paused(ctx, paused)
    }

    pub fn propose_admin(ctx: Context<Admin>, next: Pubkey) -> Result<()> {
        instructions::admin::process_propose_admin(ctx, next)
    }

    pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
        instructions::admin::process_accept_admin(ctx)
    }
}
