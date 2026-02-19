use anchor_lang::prelude::*;

use crate::states::{auction::Auction, bid::Bid};

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(
        init_if_needed,
        payer = bidder,
        space = 8 + Bid::INIT_SPACE,
        seeds = [b"bid".as_ref(), bidder.key().as_ref(), auction.key().as_ref()],
        bump
    )]
    pub bid: Account<'info, Bid>,
    pub token_mint: Account<'info, Mint>, // maybe I need to add an ATA for the token
    pub auction: Account<'info, Auction>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBid<'info> {
    pub fn place_bid(&mut self, amount: u64) -> Result<()> {
        //CHECKS
        // - is auction open
        // - has the bidder placed a bid before
        // - does the bidder have enough tokens
        //
        Ok(())
    }
}
