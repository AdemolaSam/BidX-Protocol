use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, TransferChecked},
};

use crate::states::{AuthenticatorsRegistry, PlatformConfig};

#[derive(Accounts)]
pub struct ClosePlatform<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config".as_ref(), admin.key().as_ref()],
        bump = platform_config.bump,
        has_one = admin,
        close = admin
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account(
        mut,
        seeds = [b"authenticators_registry".as_ref(), admin.key().as_ref()],
        bump = authenticators_registry.bump,
        close = admin
    )]
    pub authenticators_registry: Account<'info, AuthenticatorsRegistry>,

    #[account(
        mut,
        constraint = treasury_usdc.key() == platform_config.treasury_usdc,
    )]
    pub treasury_usdc: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = usdc_mint,
        associated_token::authority = admin,
    )]
    pub admin_usdc_ata: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClosePlatform<'info> {
    pub fn close_platform(&mut self) -> Result<()> {
        let amount = self.treasury_usdc.amount;
        let admin_key = self.admin.key();
        let seeds = &[
            b"config".as_ref(),
            admin_key.as_ref(),
            &[self.platform_config.bump],
        ];
        let signer = &[&seeds[..]];

        if amount > 0 {
            transfer_checked(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    TransferChecked {
                        from: self.treasury_usdc.to_account_info(),
                        to: self.admin_usdc_ata.to_account_info(),
                        mint: self.usdc_mint.to_account_info(),
                        authority: self.platform_config.to_account_info(),
                    },
                    signer,
                ),
                amount,
                self.usdc_mint.decimals,
            )?;
        }

        close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.treasury_usdc.to_account_info(),
                destination: self.admin.to_account_info(),
                authority: self.platform_config.to_account_info(),
            },
            signer,
        ))?;

        Ok(())
    }
}
