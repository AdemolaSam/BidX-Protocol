use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub admin: Pubkey,              // Single wallet for POC (Multisig wallet address (Squads) Later)
    pub platform_fee_bps: u16,      // Basis points (250 = 2.5%)
    pub treasury_usdc: Pubkey,      // ATA for USDC fees
    pub treasury_sol: Pubkey,       // ATA for SOL fees
    pub auth_fee_bps: u16,
    pub is_paused: bool,
    pub min_auction_duration: i64,
    pub max_auction_duration: i64,
    pub bump: u8,
}