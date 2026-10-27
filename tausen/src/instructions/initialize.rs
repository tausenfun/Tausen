use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::*, error::TausenError, events::Initialized, state::Config};

#[event_cpi]
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + Config::INIT_SPACE, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,
    pub usdc_mint: Account<'info, Mint>,
    #[account(init, payer = admin, seeds = [VAULT_SEED], bump, token::mint = usdc_mint, token::authority = config)]
    pub vault: Account<'info, TokenAccount>,
    #[account(constraint = this_program.programdata_address()? == Some(program_data.key()) @ TausenError::NotUpgradeAuthority)]
    pub this_program: Program<'info, crate::program::Tausen>,
    #[account(constraint = program_data.upgrade_authority_address == Some(admin.key()) @ TausenError::NotUpgradeAuthority)]
    pub program_data: Account<'info, ProgramData>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn process_initialize(ctx: Context<Initialize>, operator: Pubkey) -> Result<()> {
    let c = &mut ctx.accounts.config;
    c.admin = ctx.accounts.admin.key();
    c.pending_admin = None;
    c.operator = operator;
    c.usdc_mint = ctx.accounts.usdc_mint.key();
    c.vault = ctx.accounts.vault.key();
    c.last_record = 0;
    c.records = 0;
    c.journal_hash = [0; 32];
    c.deposited = 0;
    c.withdrawn = 0;
    c.paused = false;
    c.bump = ctx.bumps.config;
    c.vault_bump = ctx.bumps.vault;
    emit_cpi!(Initialized {
        admin: c.admin,
        operator,
        usdc_mint: c.usdc_mint,
        vault: c.vault
    });
    Ok(())
}
