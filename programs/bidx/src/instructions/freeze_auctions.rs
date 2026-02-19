use anchor_lang::prelude::*;

use crate::states::auction::Auction; // TODO: ENSURE CONSITENT EXPORT AND IMPORT PATTERNS

#[derive(Accounts)]
pub struct FreezeAuctions<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"auction".as_ref(), seller.key().as_ref(), auction.key().as_ref()],
        bump
    )]
    pub auction: Account<'info, Auction>,
    /// CHECK: This is safe
    pub seller: UncheckedAccount<'info>,
    pub item_vault: Account<'info, TokenAccount>,
}

impl<'info> FreezeAuctions<'info> {
    pub fn freeze_auctions(&mut self) -> Result<()> {
        Ok(())
    }
}
