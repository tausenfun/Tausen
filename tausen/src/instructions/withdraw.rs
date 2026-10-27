use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::{
    constants::*,
    error::TausenError,
    events::Withdrawn,
    state::{Config, Receipt},
};

#[event_cpi]
#[derive(Accounts)]
#[instruction(withdrawal_id: u64)]
pub struct Withdraw<'info> {
    #[account(mut, address = config.operator @ TausenError::NotOperator)]
    pub operator: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = usdc_mint @ TausenError::WrongMint, has_one = vault)]
    pub config: Account<'info, Config>,
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut, seeds = [VAULT_SEED], bump = config.vault_bump)]
    pub vault: Account<'info, TokenAccount>,
    /// CHECK: the wallet being paid; the token account below must belong to it.
    pub recipient: UncheckedAccount<'info>,
    #[account(mut, token::mint = usdc_mint, token::authority = recipient)]
    pub recipient_token: Account<'info, TokenAccount>,
    #[account(init, payer = operator, space = 8 + Receipt::INIT_SPACE, seeds = [WITHDRAWAL_SEED, &withdrawal_id.to_le_bytes()], bump)]
    pub receipt: Account<'info, Receipt>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn process_withdraw(ctx: Context<Withdraw>, withdrawal_id: u64, amount: u64, journal_seq: u64) -> Result<()> {
    require!(!ctx.accounts.config.paused, TausenError::Paused);
    require!(amount > 0, TausenError::ZeroAmount);
    let seeds: &[&[u8]] = &[CONFIG_SEED, &[ctx.accounts.config.bump]];
    token::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
                to: ctx.accounts.recipient_token.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[seeds],
        ),
        amount,
        ctx.accounts.usdc_mint.decimals,
    )?;
    let r = &mut ctx.accounts.receipt;
    r.withdrawal_id = withdrawal_id;
    r.recipient = ctx.accounts.recipient.key();
    r.amount = amount;
    r.journal_seq = journal_seq;
    r.slot = Clock::get()?.slot;
    let c = &mut ctx.accounts.config;
    c.withdrawn = c.withdrawn.checked_add(amount).ok_or(TausenError::Overflow)?;
    ctx.accounts.vault.reload()?;
    emit_cpi!(Withdrawn {
        withdrawal_id,
        recipient: r.recipient,
        amount,
        journal_seq,
        vault_balance: ctx.accounts.vault.amount
    });
    Ok(())
}
