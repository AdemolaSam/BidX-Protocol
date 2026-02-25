use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::events::AuctionCreated;
use crate::states::{AssetType, Auction, AuctionStatus, AuthStatus, Authentication, AuthenticatorsRegistry, SellerState};
use crate::errors::{ AuctionAuthError, AuctionError};

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
    pub nft_mint: InterfaceAccount<'info, Mint>,
    pub item_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = seller,
        space = 8 + Authentication::INIT_SPACE,
        seeds = [b"authentication", auction.key().as_ref()],
        bump
    )]
    pub authentication: Account<'info, Authentication>,
    #[account(mut)]
    pub registry: Account<'info, AuthenticatorsRegistry>,
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
        bumps: & CreateAuctionBumps,
        document_hash: Option<String>
    ) -> Result<()> {
        let seller_state = &mut self.seller_state;

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

        if asset_type == AssetType::PhysicalRWA {
            //Assign authenticator (using round robin)
            let registry = &mut self.registry;

            require!(
                !registry.authenticators.is_empty(),
                AuctionAuthError::NoAuthenticatorAvailable
            );

            let authenticator = registry.authenticators[registry.next_index as usize];
            //rotate to next authenticator
            registry.next_index = (registry.next_index + 1) % (registry.authenticators.len() as u64);



            self.authentication.auction = self.auction.key();
            self.authentication.authenticator = authenticator;
            self.authentication.seller = self.seller.key();
             self.authentication.auth_status = AuthStatus::Pending;
            self.authentication.report_hash = String::new();
            self.authentication.metadata_hash = document_hash.unwrap_or_default();  // Seller provides
            self.authentication.auth_status = AuthStatus::Pending;
             self.authentication.uploaded_at = 0;
            self.authentication.verified_at = 0;
            self.authentication.fee_amount = 0;
            self.authentication.fee_paid = false;
            self.authentication.bump = bumps.authentication;
        }

        let auth_status =  if asset_type == AssetType::DigitalNFT{
            AuthStatus::NotRequired
        } else {
            AuthStatus::Pending
        };

        self.auction.set_inner({
            Auction {
                seller: self.seller.key(),
                accepted_token,
                reserved_price,
                starting_bid,
                start_date,
                end_date,
                auction_status: AuctionStatus::Pending,
                auth_status,
                item_vault: self.item_vault.key(),
                nft_mint: self.nft_mint.key(),  
                asset_type,
                highest_bid: 0,
                highest_bidder: Pubkey::default(),
                bump: bumps.auction
            }
        });

        emit!(
            AuctionCreated {
                auction: self.auction.key(),
                asset_type: self.auction.asset_type.clone(),
                seller: self.seller.key(),
                timestamp: Clock::get()?.unix_timestamp
            }
        );

        Ok(())
    }
}
