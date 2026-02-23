use anchor_lang::prelude::*;

use crate::states::{AssetType, Auction, AuctionStatus, SellerState};

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        init_if_needed,
        payer = seller,
        space = 8 + SellerState::INIT_SPACE,
        seeds = [b"seller_state", seller.key().as_ref()],
        bump
    )]
    pub seller_state: Account<'info, SellerState>,

    #[account(
        init,
        payer = seller,
        space = 8 + Auction::INIT_SPACE,
        seeds = [b"auction".as_ref(), seller.key().as_ref(), auction.key().as_ref()], // is the auction key sufficient for unique auction creation or do I need to include the auction ID?
        bump
    )]
    pub auction: Account<'info, Auction>,
    pub nft_mint: Account<'info, Mint>,
    pub item_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateAuction<'info> {
    pub fn create(
        &mut self,
        accepted_token: Pubkey,
        starting_bid: u64,
        reserved_price: u64,
        start_date: i64,
        end_date: i64,
        asset_type: AssetType
    ) -> Result<()> {
        // I NEED TO MAKE THE START AND END DATES OPTIONAL
        // CHECKS
        self.auction.set_inner({
            Auction {
                seller: self.seller.key(),
                accepted_token: accepted_token.key(),
                reserved_price,
                starting_bid,
                start_date,
                end_date,
                auction_status: AuctionStatus::Pending,
                auth_status: AuctionStatus::Pending, //IF NFT "NOTREQUIRED"
                item_vault: self.item_vault.key(),
                nft_mint: self.nft_mint.key(),  
                asset_type,
            }
        });
        Ok(())
    }
}
