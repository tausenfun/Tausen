use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::{constants::*, error::TausenError, events::Deposited, state::Config};

#[event_cpi]
#[derive(Accounts)]
pub struct Deposit<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = usdc_mint @ TausenError::WrongMint, has_one = vault)]
    pub config: Account<'info, Config>,
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut, token::mint = usdc_mint, token::authority = user)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut, seeds = [VAULT_SEED], bump = config.vault_bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn process_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(!ctx.accounts.config.paused, TausenError::Paused);
    require!(amount > 0, TausenError::ZeroAmount);
    token::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.user_token.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.usdc_mint.decimals,
    )?;
    let c = &mut ctx.accounts.config;
    c.deposited = c.deposited.checked_add(amount).ok_or(TausenError::Overflow)?;
    ctx.accounts.vault.reload()?;
    emit_cpi!(Deposited {
        user: ctx.accounts.user.key(),
        amount,
        vault_balance: ctx.accounts.vault.amount
    });
    Ok(())
}
