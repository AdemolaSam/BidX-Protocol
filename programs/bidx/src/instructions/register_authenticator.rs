use anchor_lang::prelude::*;

use crate::states::AuthenticatorsRegistry;

#[derive(Accounts)]
pub struct RegisterAuthenticator<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"authenticators_registry".as_ref()],
        bump
    )]
    pub registry: Account<'info, AuthenticatorsRegistry>,
}

impl<'info> RegisterAuthenticator<'info> {
    pub fn register_authenticator(&mut self, authenticator: Pubkey) -> Result<()> {
        //CHECKS
        // is authenticator already registered?
        // is the authenticator a valid keypair?
        // is the authenticator an admin (if yes decline)
        Ok(())
    }
}
