use anchor_lang::prelude::*;

use crate::events::AuthenticatorRemovedFromPlatform;
use crate::states::AuthenticatorsRegistry;
use crate::errors::ConfigError;

#[derive(Accounts)]
pub struct RemoveAuthenticator<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"authenticators_registry".as_ref()],
        bump
    )]
    pub registry: Account<'info, AuthenticatorsRegistry>,
}

impl<'info> RemoveAuthenticator<'info> {
    pub fn remove_authenticator(&mut self, authenticator: Pubkey) -> Result<()> {
       
        require!(
            self.admin.key() == self.registry.admin.key(),
            ConfigError::ExclusiveToAdmin
        );

        // is authenticator registered?
        require!(
            self.registry.authenticators.contains(&authenticator),
            ConfigError::AuthenticatorNotInRegistry
        );

        // WARNING: Removing an authenticator with pending verifications
        // will cause those auctions to become stuck in "Pending" state.
        // Sellers will need to cancel and relist.
        // Admin should coordinate off-chain before removal.
        let auth_index = self.registry.authenticators.iter().position(|& el| el == authenticator).unwrap();

        self.registry.authenticators.swap_remove(auth_index);

        emit!(
            AuthenticatorRemovedFromPlatform {
                authenticator,
                timestamp: Clock::get()?.unix_timestamp
            }
        );

        Ok(())
    }
}
