use anchor_lang::prelude::*;

use crate::{
    errors::AuctionAuthError, 
    events::AuthenticationResolved, 
    states::{Auction, AuctionStatus, AuthStatus, Authentication, AuthenticatorsRegistry}
};


#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct AttestAuthentication<'info> {
    #[account(mut)]
    pub authenticator: Signer<'info>,

    ///CHECK: seller
    pub seller: UncheckedAccount<'info>,

    #[account(
        seeds = [b"auction", seller.key().as_ref(), &nonce.to_le_bytes()],
        bump
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        seeds = [b"authentication", auction.key().as_ref()],
        bump,
        has_one = authenticator
    )]
    pub authentication: Account<'info, Authentication>,

    #[account(
        seeds = [b"authenticators_registry", registry.admin.as_ref()],
        bump
    )]
    pub registry: Account<'info, AuthenticatorsRegistry>
}


impl <'info> AttestAuthentication <'info> {
    pub fn attest_authentication(&mut self, approved: bool) -> Result<()> {
        // validate authenticator
        require!(
            self.registry.authenticators.contains(&self.authenticator.key()),
            AuctionAuthError::AuthenticatorNotRecognized
        );

        // validate report was uploaded
        require!(
            !self.authentication.report_hash.is_empty(),
            AuctionAuthError::ReportNotUploaded
        );

        if approved {
            self.authentication.auth_status = AuthStatus::Verified;
            self.auction.auth_status = AuthStatus::Verified;
            self.auction.auction_status = AuctionStatus::Active;
        } else {
            self.authentication.auth_status = AuthStatus::Rejected;
            self.auction.auth_status = AuthStatus::Rejected;
            self.auction.auction_status = AuctionStatus::Cancelled;
        }
        self.authentication.verified_at = Clock::get()?.unix_timestamp;

        emit!(
            AuthenticationResolved {
                accepted: approved,
                authentication: self.authentication.key(),
                authenticator: self.authenticator.key(),
                verified_at: self.authentication.verified_at,
            }
        );

        Ok(())
    }
}
