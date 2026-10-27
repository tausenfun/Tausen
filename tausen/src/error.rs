use anchor_lang::prelude::*;

#[error_code]
pub enum TausenError {
    #[msg("Deposits and withdrawals are paused")]
    Paused,
    #[msg("The amount must be more than zero")]
    ZeroAmount,
    #[msg("Only the operator can do this")]
    NotOperator,
    #[msg("Only the admin can do this")]
    NotAdmin,
    #[msg("Only the proposed admin can accept the role")]
    NotProposedAdmin,
    #[msg("Only the program's upgrade authority can initialize it")]
    NotUpgradeAuthority,
    #[msg("Entries must be recorded in increasing order")]
    OutOfOrder,
    #[msg("That kind of entry doesn't exist")]
    UnknownKind,
    #[msg("That is not the vault's USDC mint")]
    WrongMint,
    #[msg("Amount overflow")]
    Overflow,
}
