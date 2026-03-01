use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{
    events::PlatformInitialized,
    states::{AuthenticatorsRegistry, PlatformConfig}
};

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + PlatformConfig::INIT_SPACE,
        seeds = [b"config".as_ref(), admin.key().as_ref()],
        bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + AuthenticatorsRegistry::INIT_SPACE,
        seeds = [b"authenticators_registry".as_ref(), admin.key().as_ref()],
        bump
    )]
    pub authenticators_registry: Account<'info, AuthenticatorsRegistry>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = usdc_mint,
        associated_token::authority = platform_config,
    )]
    pub treasury_usdc: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = wsol_mint,
        associated_token::authority = wsol_mint
    )]
    pub treasury_sol: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,
    pub wsol_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePlatform<'info> {
    pub fn initialize_platform(
        &mut self,
        platform_fee_bps: u16,
        auth_fee_bps: u16,
        min_auction_duration: i64,
        max_auction_duration: i64,
        authenticators: Vec<Pubkey>,
        bumps: &InitializePlatformBumps
    ) -> Result<()> {
        self.platform_config.set_inner(
            PlatformConfig {
                admin: self.admin.key(),
                min_auction_duration,
                max_auction_duration,
                is_paused: false,
                platform_fee_bps,
                auth_fee_bps,
                treasury_sol: self.treasury_sol.key(),
                treasury_usdc: self.treasury_usdc.key(),
                bump: bumps.platform_config
        });

        self.authenticators_registry.set_inner({
            AuthenticatorsRegistry {
                admin: self.admin.key(),
                authenticators,
                next_index: 0,
                bump: bumps.authenticators_registry
            }
        });

        emit!(PlatformInitialized {
            message: String::from("Platform Initialized"),
            admin: self.admin.key(),
            total_authenticators: self.authenticators_registry.authenticators.len() as u64,  
        });

        Ok(())
    }
}
