use anchor_lang::prelude::*;

use crate::states::{AssetType, Auction, AuctionStatus, SellerState};
use crate::errors::{ AuctionError};

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
        seeds = [b"auction".as_ref(), seller.key().as_ref(), &seller_state.auction_count.to_le_bytes()],
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
        asset_type: AssetType,
        bumps: & CreateAuctionBumps
    ) -> Result<()> {
        let seller_state = &self.seller_state;

        //checks
        require!(start_date > Clock::get()?.unix_timestamp, AuctionError::StartDateIsBehind);
        require!(end_date > start_date, AuctionError::EndDateIsBehindStartDate);
        require!(reserved_price > starting_bid, AuctionError::ReservedPriceTooLow);
        
        // first auction creation
        if seller_state.auction_count == 0 {
            seller_state.seller = self.seller.key();
            seller_state.bump = bumps.seller_state;
        }

        self.seller_state.auction_count += 1;

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
                highest_bid: 0,
                highest_bidder: Pubkey::default(),
                bump: bumps.auction
            }
        });
        Ok(())
    }
}
