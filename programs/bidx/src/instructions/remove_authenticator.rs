use anchor_lang::prelude::*;

use crate::states::AuthenticatorsRegistry;

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
        //CHECKS
        // is authenticator registered?
        // is the authenticator a valid keypair?
        // is the authenticator currently verifying an auction? (replace authenticator, notify seller)
        Ok(())
    }
}
