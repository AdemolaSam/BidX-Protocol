use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub admin: Pubkey, // is single admin multi sig? How do I allow multiple admins?
    pub platform_fee: u64,
    pub treasury: Pubkey,
    pub treasury_mint: Pubkey,
    pub platform_fee_token_decimals: u8,
    pub is_paused: bool,
    pub min_auction_duration: u64,
    pub max_auction_duration: u64,
    pub bump: u8,
}
