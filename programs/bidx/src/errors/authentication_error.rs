use anchor_lang::prelude::error_code;


#[error_code]
pub enum AuctionAuthError {
    #[msg("Uploaded review hash is required before verdict")]
    ReviewHashNotProvided,

    #[msg("Only Assigned Authenticators can update AuctionAuth")]
    Unauthorized,
}