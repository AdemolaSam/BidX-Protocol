use anchor_lang::prelude::*;

use crate::states::{AuthenticatorsRegistry, PlatformConfig};

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + PlatformConfig::INIT_SPACE,
        seeds = [b"config".as_ref()],
        bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    #[account(
        init,
        payer = admin,
        space = AuthenticatorsRegistory::Discriminator + AuthenticatorsRegistory::INIT_SPACE,
        seeds = [b"authenticators_registry".as_ref()],
        bump
    )]
    pub authenticators_registory: Account<'info, AuthenticatorsRegistry>,
    #[account(
        mut,
        seeds = [b"treasury".as_ref(), platform_config.key().as_ref(), admin.key().as_ref()],
        bump
    )]
    pub treasury: UncheckedAccount<'info>,
    pub treasury_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePlatform<'info> {
    pub fn initialize_platform(&mut self, authenticators: Vec<Pubkey>) -> Result<()> {
        Ok(())
    }
}
