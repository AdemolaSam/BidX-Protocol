use anchor_lang::prelude::*;

use crate::states::Auction;

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
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
    pub fn new(
        &mut self,
        starting_bid: u64,
        reserved_price: u64,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        // I NEED TO MAE THE START AND END DATES OPTIONAL
        Ok(())
    }
}
