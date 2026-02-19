use anchor_lang::prelude::*;

use crate::states::{auction::Auction, authentication::Authentication};

// THIS CODE INVOLVES UPDATING THE AUTHENTICATION STATE OF AN AUCTION. IT REQUIRES THE AUTHENTICATOR TO UPLOAD A REPORT TO IPFS BEFORE ACCEPTING OR REJECTING THE AUCTION.
//
#[derive(Accounts)]
pub struct UpdateAuthentication<'info> {
    #[account(mut)]
    pub authenticator: Signer<'info>,
    #[account(
        mut,
        has_one = seller,
        seeds = [b"auction", seller.key().as_ref(), auction.key().as_ref()],// The auction key may be replaced by auction id later or included along side
        bump
    )]
    pub auction: Account<'info, Auction>,
    /// CHECK: This is a valid seller account
    pub seller: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateAuthentication<'info> {
    pub fn update_authentication(&mut self) -> Result<()> {
        Ok(())
    }
}
