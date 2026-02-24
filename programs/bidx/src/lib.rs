use anchor_lang::prelude::*;

mod instructions;
mod states;
mod errors;

declare_id!("2skNLUQeMc1ZBKPPXEuUEms2WvREu2TpVT5R7JvWzNVm");

#[program]
pub mod bidx {

    use crate::{instructions::{
        CreateAuction, CreateAuctionBumps, FreezeAuctions, InitializePlatform, InitializePlatformBumps, PlaceBid, PlaceBidBumps, RegisterAuthenticators, RemoveAuthenticator, UpdateAuthentication
    }, states::AssetType};

    use super::*;

    pub fn initialize(
        ctx: Context<InitializePlatform>,
        platform_fee_bps: u16,
        min_auction_duration: i64,
        max_auction_duration: i64,
        authenticators: Vec<Pubkey>,
        bumps: InitializePlatformBumps
    ) -> Result<()> {
        ctx.accounts.initialize_platform(platform_fee_bps, min_auction_duration, max_auction_duration, authenticators, &bumps)?;
        Ok(())
    }

    pub fn creeate_auction(
        ctx: Context<CreateAuction>,
        accepted_token: Pubkey,
        starting_bid: u64,
        reserved_price: u64,
        start_date: i64,
        end_date: i64,
        asset_type: AssetType,
    ) -> Result<()> {
        ctx.accounts.create(accepted_token, starting_bid, reserved_bid, start_date, end_date, asset_type, &CreateAuctionBumps)?;
        Ok(())
    }

    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64, bumps: &PlaceBidBumps) -> Result<()> {
        ctx.accounts.place_bid(amount, bumps)?;
        Ok(())
    }

    pub fn register_authenticators(
        ctx: Context<RegisterAuthenticators>,
        authenticators: vec<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.register_authenticators(authenticators)
    }

    pub fn remove_authenticator(
        ctx: Context<RemoveAuthenticator>,
        authenticator: Pubkey,
    ) -> Result<()> {
        ctx.accounts.remove_authenticator(authenticator)?;
        Ok(())
    }

    pub fn freeze_auctions(ctx: Context<FreezeAuctions>) -> Result<()> {
        ctx.accounts.freeze_auctions()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
