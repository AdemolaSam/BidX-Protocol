use anchor_lang::prelude::*;

use crate::states::AuthStatus;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum AssetType {
    DigitalNFT,
    PhysicalRWA,
}

#[derive(Debug, Clone, InitSpace, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum AuctionStatus {
    Pending,
    Paused,
    Active,
    Ended,
    Settled,
    Cancelled,
    Failed,
}

#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub item_vault: Pubkey,
    pub asset_type: AssetType,
    pub starting_bid: u64,
    pub reserved_price: u64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub accepted_token: Pubkey,
    pub start_date: i64,
    pub end_date: i64,
    pub auction_status: AuctionStatus,
    pub auth_status: AuthStatus,
    pub bump: u8
}
