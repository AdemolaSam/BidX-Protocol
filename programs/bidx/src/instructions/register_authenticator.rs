use anchor_lang::prelude::*;


use crate::events::AuthenticatorsAddedToPlatform;
use crate::states::AuthenticatorsRegistry;
use crate::errors::{AuctionAuthError, ConfigError};

#[derive(Accounts)]
pub struct RegisterAuthenticators<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"authenticators_registry".as_ref(), registry.admin.as_ref()],
        bump
    )]
    pub registry: Account<'info, AuthenticatorsRegistry>,
}

impl<'info> RegisterAuthenticators<'info> {
    pub fn register_authenticators(&mut self, authenticators: Vec<Pubkey>) -> Result<()> {
        // ensure caller is admin
        require!(
            self.admin.key() == self.registry.admin.key(),
            ConfigError::ExclusiveToAdmin
        );
        for authenticator in &authenticators {

            //check if authenticator key is valid/not null
            require!(
                *authenticator != Pubkey::default(),
                AuctionAuthError::InvalidKey
            );

            require!(
                !self.registry.authenticators.contains(&authenticator),
                AuctionAuthError::AlreadyRegistered
            );

            require!(
                self.admin.key() != *authenticator,
                ConfigError::AdminCannotbeAuthenticator
            );
        
            self.registry.authenticators.push(authenticator.clone());
            
        }

        emit!(
            AuthenticatorsAddedToPlatform {
                total_added: authenticators.len() as u64,
                timestamp: Clock::get()?.unix_timestamp
            }
        );
        
        Ok(())
    }
}
