use anchor_lang::prelude::*;

mod instructions;
mod states;
mod errors;
mod events;

declare_id!("2skNLUQeMc1ZBKPPXEuUEms2WvREu2TpVT5R7JvWzNVm");

#[program]
pub mod bidx {

    use crate::{instructions::{AttestAuthentication,
        CreateAuction, CreateAuctionBumps, InitializePlatform, InitializePlatformBumps, PlaceBid, PlaceBidBumps, RegisterAuthenticators, RemoveAuthenticator, SettleAuction, TogglePause, UpdateAuthentication, UpdatePlatformConfig, UploadAuthDocument, WithdrawBid
    }, states::AssetType};

    use super::*;

    pub fn initialize(
        ctx: Context<InitializePlatform>,
        platform_fee_bps: u16,
        min_auction_duration: i64,
        max_auction_duration: i64,
        authenticators: Vec<Pubkey>,
        auth_fee_bps: u16,
        bumps: InitializePlatformBumps
    ) -> Result<()> {
        ctx.accounts.initialize_platform(
            platform_fee_bps,
            auth_fee_bps,
            min_auction_duration,
            max_auction_duration,
            authenticators,
            &InitializePlatformBumps
        )
    }

    pub fn creeate_auction(
        ctx: Context<CreateAuction>,
        accepted_token: Pubkey,
        starting_bid: u64,
        reserved_price: u64,
        start_date: i64,
        end_date: i64,
        document_hash: Option<String>,
        asset_type: AssetType,
    ) -> Result<()> {
        ctx.accounts.create(
            accepted_token,
            starting_bid,
            reserved_bid,
            start_date,
            end_date,
            asset_type,
            &CreateAuctionBumps,
            document_hash,
        )
    }

    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64, bumps: &PlaceBidBumps) -> Result<()> {
        ctx.accounts.place_bid(amount, bumps)
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
        ctx.accounts.remove_authenticator(authenticator)
    }

    pub fn upload_auth_report(
        ctx: Context<UploadAuthDocument>,
        report_hash: String,
    ) -> Result<()>{
        ctx.accounts.upload_auth_document(report_hash)
    }

    pub fn attest_authentication(
        ctx: Context<AttestAuthentication>,
        approved: bool) -> Result<()> {
        ctx.accounts.attest_authentication(approved)
    }

    pub fn settle_auction(ctx: Context<SettleAuction>) -> Result<()> {
        ctx.accounts.settle_auction()
    }

    pub fn withdraw_bid(ctx: Context<WithdrawBid>) -> Result<()> {
        ctx.accounts.withdraw_bid()
    }

    pub fn toggle_pause_platform(ctx: Context<TogglePause>) -> Result<()>{
        ctx.accounts.toggle_pause()
    }

    pub fn update_platform_config(ctx: Context<UpdatePlatformConfig>,
        platform_fee_bps: Option<u16>,
        auth_fee_bps: Option<u16>,
        min_auction_duration: Option<i64>,
        max_auction_duration: Option<i64>
     ) -> Result<()> {
        ctx.accounts.update_platform_config(
            platform_fee_bps,
            auth_fee_bps,
            min_auction_duration,
            max_auction_duration
        )
    }
}

#[derive(Accounts)]
pub struct Initialize {}
