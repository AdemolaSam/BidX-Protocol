use anchor_lang::prelude::*;
use anchor_spl::token::{CloseAccount, Mint, Token, TokenAccount, TransferChecked, close_account, transfer_checked};

use crate::events::BidWithdrawn;
use crate::states::{Auction, AuctionStatus, Bid};
use crate::errors::BidError;


#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct WithdrawBid <'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    ///CHECK: seller account 
    pub seller: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"auction", seller.key().as_ref(), &nonce.to_le_bytes()],
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        seeds = [b"bid", bidder.key().as_ref(), auction.key().as_ref()],
        bump = bid.bump,
        has_one = bidder,
        has_one = auction,
        close = bidder
    )]
    pub bid: Account<'info, Bid>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = bid,
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = bidder,
    )]
    pub bidder_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> WithdrawBid <'info> {
    pub fn withdraw_bid(&mut self) -> Result<()> {
        let bid = &self.bid;
        let auction = &self.auction;

        require!(
            self.auction.auction_status == AuctionStatus::Ended
            && self.auction.highest_bidder != self.bidder.key(),
            BidError::StillWinning
        );

        //PDA signer
        let bidder_key = &self.bidder.key();
        let auction_key = auction.key();

        let seeds = &[
            b"bid",
            bidder_key.as_ref(),
            auction_key.as_ref(),
            &[bid.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.escrow_vault.to_account_info(),
                to: self.bidder_token_account.to_account_info(),
                mint: self.token_mint.to_account_info(),
                authority: self.bid.to_account_info()
            },
            signer_seeds
        );

        transfer_checked(
            cpi_ctx,
            self.bid.amount,
            self.token_mint.decimals
        );

        //Close escrow account
        close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                CloseAccount {
                    account: self.escrow_vault.to_account_info(),
                    destination: self.bidder.to_account_info(),
                    authority: self.bid.to_account_info(),
                },
                signer_seeds
            )
        )?;

        emit!(
            BidWithdrawn {
                auction: self.auction.key(),
                bidder: self.bidder.key(),
                ammount: self.bid.amount
            }
        );

        Ok(())

    }
}


