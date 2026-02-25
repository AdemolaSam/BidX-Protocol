use anchor_lang::prelude::*;

use crate::events::BidPlaced;
use crate::states::{AuctionStatus, auction::Auction, bid::Bid};
use crate::errors::{BidError};

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        transfer_checked, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = bidder,
        space = 8 + Bid::INIT_SPACE,
        seeds = [b"bid", bidder.key().as_ref(), auction.key().as_ref()],
        bump
    )]
    pub bid: Account<'info, Bid>,
    
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = bidder,
        associated_token::token_program = token_program
    )]
    pub bidder_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = bidder,
        associated_token::mint = token_mint,
        associated_token::authority = bid,
        associated_token::token_program = token_program
    )]
    pub escrow_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBid<'info> {
    pub fn place_bid(&mut self, amount: u64, bumps: &PlaceBidBumps) -> Result<()> {
        // Validations
        require!(
            self.auction.auction_status == AuctionStatus::Active,
            BidError::AuctionNotAvailable
        );
        require!(
            self.token_mint.key() == self.auction.accepted_token,
            BidError::WrongToken
        );
        
        // Check if increasing existing bid or new bid
        if self.bid.bidder != Pubkey::default() {
            // Increasing existing bid
            let new_total = self.bid.amount + amount;
            require!(
                new_total > self.auction.highest_bid,
                BidError::BidTooLow
            );
            
            self.bid.amount = new_total;
            self.bid.time_stamp = Clock::get()?.unix_timestamp;
        } else {
            // First bid
            require!(
                amount > self.auction.highest_bid,
                BidError::BidTooLow
            );
            
            self.bid.set_inner(Bid {
                amount,
                auction: self.auction.key(),
                bidder: self.bidder.key(),
                is_active: true,
                is_winner: false,
                time_stamp: Clock::get()?.unix_timestamp,
                token_mint: self.token_mint.key(),
                bump: bumps.bid,
            });
        }
        
        // Transfer tokens to escrow
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.bidder_token_account.to_account_info(),
                    to: self.escrow_vault.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                    authority: self.bidder.to_account_info(),
                },
            ),
            amount,
            self.token_mint.decimals,
        )?;
        
        // Update auction state
        self.auction.highest_bid = self.bid.amount;
        self.auction.highest_bidder = self.bidder.key();

        emit!(
            BidPlaced {
                auction: self.auction.key(),
                bid_amount: amount,
                bidder: self.bidder.key(),
                timestamp: Clock::get()?.unix_timestamp,
            }
        );
        
        Ok(())
    }
}