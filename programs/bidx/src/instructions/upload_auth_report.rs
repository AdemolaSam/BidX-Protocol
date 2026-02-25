use anchor_lang::prelude::*;

use crate::events::AuthReportUploaded;
use crate::states::{Auction, AuthStatus, Authentication, AuthenticatorsRegistry};
use crate::errors::AuctionAuthError;


#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct UploadAuthDocument <'info> {
    #[account(mut)]
    pub authenticator: Signer<'info>,

    #[account(
        seeds = [b"auction", auction.seller.as_ref(), &nonce.to_le_bytes()],
        bump
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        seeds = [b"authentication", auction.key().as_ref()],
        has_one = auction,
        bump
    )]
    pub authentication: Account<'info, Authentication>,
    #[account(
        seeds = [b"authenticators_registry".as_ref()],
        bump
    )]
    pub registry: Account<'info, AuthenticatorsRegistry>
}

impl <'info> UploadAuthDocument<'info> {
    pub fn upload_auth_document(&mut self, report_hash: String) -> Result<()> {
        // validate authenticator is registered
        require!(
            self.registry.authenticators.contains(&self.authenticator.key()),
            AuctionAuthError::AuthenticatorNotRecognized
        );

        require!(
            self.auction.auth_status == AuthStatus::Pending,
            AuctionAuthError::NotPending
        );

        self.authentication.authenticator = self.authenticator.key();
        self.authentication.report_hash = report_hash;
        self.authentication.uploaded_at = Clock::get()?.unix_timestamp;

        emit!(
            AuthReportUploaded {
                authentication: self.authentication.key(),
                authenticator: self.authenticator.key(),
                report_hash: self.authentication.report_hash,
                uploaded_at: self.authentication.uploaded_at
            }
        );
        
        Ok(())
    }
}