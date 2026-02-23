use anchor_lang::prelude::*;

mod instructions;
mod states;

declare_id!("2skNLUQeMc1ZBKPPXEuUEms2WvREu2TpVT5R7JvWzNVm");

#[program]
pub mod bidx {

    use crate::instructions::{
        CreateAuction, FreezeAuctions, InitializePlatform, RegisterAuthenticator, PlaceBid,
        RemoveAuthenticator, UpdateAuthentication,
    };

    use super::*;

    pub fn initialize(ctx: Context<InitializePlatform>, authenticators: Vec<Pubkey>) -> Result<()> {
        ctx.accounts.initialize_platform(authenticators)?;
        Ok(())
    }

    pub fn creeate_auction(
        ctx: Context<CreateAuction>,
        starting_bid: u64,
        reserved_price: u64,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        ctx.accounts.create(starting_bid, reserved_bid, start_date, end_date)?;
        Ok(())
    }

    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64) -> Result<()> {
        ctx.accounts.place_bid(amount)?;
        Ok(())
    }

    pub fn register_authenticator(
        ctx: Context<RegisterAuthenticator>,
        authenticator: Pubkey,
    ) -> Result<()> {
        ctx.accounts.register_authenticator(authenticator)
    }

    pub fn remove_authenticator(
        ctx: Context<RemoveAuthenticator>,
        authenticator: Pubkey,
    ) -> Result<()> {
        ctx.accounts.remove_authenticator(authenticator)?;
        Ok(())
    }

    pub fn update_authentication(ctx: Context<UpdateAuthentication>) -> Result<()> {
        ctx.accounts.update_authentication()?;
        Ok(())
    }

    pub fn freeze_auctions(ctx: Context<FreezeAuctions>) -> Result<()> {
        ctx.accounts.freeze_auctions()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
