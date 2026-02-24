use anchor_lang::prelude::*;

use crate::states::AssetType;

#[event]
pub struct AuctionSettled {
    pub auction: Pubkey,
    pub winner: Pubkey,
    pub final_price: u64,
    pub platform_fee: u64,
    pub auth_fee: u64,
    pub seller_amount: u64,
}

#[event]
pub struct AuctionCreatedEvent {
    pub auction: Pubkey,
    pub asset_type: AssetType,
    pub seller: Pubkey,
    pub timestamp: i64,
}