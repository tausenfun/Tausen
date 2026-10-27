use anchor_lang::prelude::*;

use crate::{
    constants::*,
    error::TausenError,
    events::Recorded,
    state::{Config, Entry, EntryKind},
};

#[event_cpi]
#[derive(Accounts)]
pub struct Record<'info> {
    #[account(address = config.operator @ TausenError::NotOperator)]
    pub operator: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, Config>,
}

pub fn process_record(ctx: Context<Record>, entry: Entry) -> Result<()> {
    require!(entry.kind < EntryKind::COUNT, TausenError::UnknownKind);
    let c = &mut ctx.accounts.config;
    require!(entry.seq > c.last_record, TausenError::OutOfOrder);
    let chain = entry.chain(&c.journal_hash);
    c.last_record = entry.seq;
    c.records = c.records.checked_add(1).ok_or(TausenError::Overflow)?;
    c.journal_hash = chain;
    emit_cpi!(Recorded {
        seq: entry.seq,
        kind: entry.kind,
        wallet: entry.wallet,
        token: entry.token,
        amount: entry.amount,
        data_hash: entry.data_hash,
        at: entry.at,
        chain,
    });
    Ok(())
}
