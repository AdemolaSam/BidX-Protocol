use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SellerState {
    pub seller: Pubkey,
    pub auction_count: u64,
    pub bump: u8
}