use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

#[derive(Debug, Clone, InitSpace, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AuctionStatus {
    Pending,
    Paused,
    Active,
    Ended,
    Settled,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, InitSpace, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AssetType {
    DigitalNFT,
    PhysicalRWA
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
