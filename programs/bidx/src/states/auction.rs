use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

#[derive(Debug, Clone, InitSpace, BorshSerialize, BorshDeserialize)]
enum AuctionStatus {
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
    // Should I have different platform config per auction?
    pub nft_mint: Pubkey,
    pub item_vault: Pubkey,
    pub starting_bid: u64,
    pub reserved_price: u64,
    pub start_date: i64,
    pub end_date: i64,
    pub auction_status: AuctionStatus,
    pub auth_status: AuthStatus,
}
