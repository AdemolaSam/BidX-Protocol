use anchor_lang::prelude::*;

use crate::errors::AuctionError;
use crate::states::{Auction, AuctionStatus};

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct EndAuction<'info> {
    /// CHECK: seller account for PDA seeds
    pub seller: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"auction", seller.key().as_ref(), &nonce.to_le_bytes()],
        bump = auction.bump,
        has_one = seller
    )]
    pub auction: Account<'info, Auction>,
}

impl<'info> EndAuction<'info> {
    pub fn end_auction(&mut self) -> Result<()> {
        if self.auction.auction_status == AuctionStatus::Ended {
            return Ok(());
        }

        require!(
            self.auction.auction_status == AuctionStatus::Active,
            AuctionError::AuctionNotEnded
        );

        let now = Clock::get()?.unix_timestamp;
        require!(now >= self.auction.end_date, AuctionError::AuctionNotEnded);

        self.auction.auction_status = AuctionStatus::Ended;
        Ok(())
    }
}
