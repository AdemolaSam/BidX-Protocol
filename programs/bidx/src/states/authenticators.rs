use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AuthenticatorsRegistry {
    pub admin: Pubkey,
    #[max_len(100)]
    pub authenticators: Vec<Pubkey>,
    pub next_index: u64,
    pub bump: u8,
}
