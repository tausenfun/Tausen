use anchor_lang::prelude::*;

use crate::{
    constants::*,
    error::TausenError,
    events::{AdminChanged, AdminProposed, OperatorChanged, PausedChanged},
    state::Config,
};

#[event_cpi]
#[derive(Accounts)]
pub struct Admin<'info> {
    #[account(address = config.admin @ TausenError::NotAdmin)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, Config>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct AcceptAdmin<'info> {
    #[account(constraint = config.pending_admin == Some(next.key()) @ TausenError::NotProposedAdmin)]
    pub next: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, Config>,
}

pub fn process_set_operator(ctx: Context<Admin>, operator: Pubkey) -> Result<()> {
    let c = &mut ctx.accounts.config;
    let previous = c.operator;
    c.operator = operator;
    emit_cpi!(OperatorChanged { previous, operator });
    Ok(())
}

pub fn process_set_paused(ctx: Context<Admin>, paused: bool) -> Result<()> {
    ctx.accounts.config.paused = paused;
    emit_cpi!(PausedChanged { paused });
    Ok(())
}

pub fn process_propose_admin(ctx: Context<Admin>, next: Pubkey) -> Result<()> {
    ctx.accounts.config.pending_admin = Some(next);
    emit_cpi!(AdminProposed { next });
    Ok(())
}

pub fn process_accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
    let c = &mut ctx.accounts.config;
    let previous = c.admin;
    c.admin = ctx.accounts.next.key();
    c.pending_admin = None;
    emit_cpi!(AdminChanged {
        previous,
        admin: c.admin
    });
    Ok(())
}
