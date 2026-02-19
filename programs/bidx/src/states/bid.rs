use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bid {
    pub bidder: Pubkey,
    pub amount: u64,
    pub auction: Pubkey,
    pub token_mint: Pubkey,
    pub time_stamp: i64,
    pub is_active: bool,
    pub is_winner: bool,
    pub bump: u8,
}
