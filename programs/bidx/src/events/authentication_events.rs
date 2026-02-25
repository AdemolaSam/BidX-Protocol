use anchor_lang::prelude::*;

#[event]
pub struct AuthenticationRequested {
    pub auction: Pubkey,
    pub seller: Pubkey,
}

#[event]
pub struct AuthReportUploaded {
    pub authentication: Pubkey,
    pub report_hash: String,
    pub uploaded_at: i64,
    pub authenticator: Pubkey,
}


#[event]
pub struct AuthenticationResolved {
    pub authentication: Pubkey,
    pub authenticator: Pubkey,
    pub accepted: bool,
    pub verified_at: i64,
}

#[event]
pub struct BidPlaced {
    pub auction: Pubkey,
    pub bidder: Pubkey,
    pub bid_amount: u64,
    pub timestamp: i64,
}
