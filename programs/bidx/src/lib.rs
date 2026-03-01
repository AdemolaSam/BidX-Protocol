use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod errors;
pub mod events;

pub use instructions::*;
pub use states::*;
pub use events::*;
pub use errors::*;



declare_id!("2skNLUQeMc1ZBKPPXEuUEms2WvREu2TpVT5R7JvWzNVm");

#[program]
pub mod bidx {

    use super::*;

    pub fn initialize(
        ctx: Context<InitializePlatform>,
        platform_fee_bps: u16,
        min_auction_duration: i64,
        max_auction_duration: i64,
        authenticators: Vec<Pubkey>,
        auth_fee_bps: u16,
    ) -> Result<()> {
        ctx.accounts.initialize_platform(
            platform_fee_bps,
            auth_fee_bps,
            min_auction_duration,
            max_auction_duration,
            authenticators,
           &ctx.bumps
        )
    }

    pub fn create_auction(
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
            reserved_price,
            start_date,
            end_date,
            asset_type,
            &ctx.bumps,
            document_hash,
        )
    }

    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64) -> Result<()> {
        ctx.accounts.place_bid(amount, &ctx.bumps)
    }

    pub fn end_auction(ctx: Context<EndAuction>, nonce: u64) -> Result<()> {
        let _ = nonce; // nonce used in account constraints
        ctx.accounts.end_auction()
    }

    pub fn register_authenticators(
        ctx: Context<RegisterAuthenticators>,
        authenticators: Vec<Pubkey>,
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
        ctx: Context<UploadAuthReport>,
        report_hash: String,
    ) -> Result<()>{
        ctx.accounts.upload_auth_report(report_hash)
    }

    pub fn attest_authentication(
        ctx: Context<AttestAuthentication>,
        approved: bool) -> Result<()> {
        ctx.accounts.attest_authentication(approved)
    }

    pub fn settle_auction(ctx: Context<SettleAuction>, nonce:u64) -> Result<()> {
        ctx.accounts.settle_auction(nonce)
    }

    pub fn withdraw_bid(ctx: Context<WithdrawBid>, nonce: u64) -> Result<()> { // NOTE: include nonce to satisfy WithdrawBid PDA seeds
        let _ = nonce; // NOTE: nonce used in account constraints; keep arg to align client + IDL
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

    pub fn close_platform(ctx: Context<ClosePlatform>) -> Result<()> {
        ctx.accounts.close_platform()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
