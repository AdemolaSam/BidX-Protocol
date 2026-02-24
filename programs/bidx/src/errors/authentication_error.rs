use anchor_lang::prelude::error_code;


#[error_code]
pub enum AuctionAuthError {
    #[msg("Invalid Authenticator Pubkey")]
    InvalidKey,
    #[msg("Authenticator not recognized")]
    AuthenticatorNotRecognized,
    #[msg("Auction Auth Status Not Pending")]
    NotPending,
    #[msg("Uploaded review hash is required before verdict")]
    ReviewHashNotProvided,
    #[msg("Only Assigned Authenticators can update AuctionAuth")]
    Unauthorized,
    #[msg("Authenticator already registered")]
    AlreadyRegistered,
    #[msg("Authentication report not uploaded")]
    ReportNotUploaded,
    #[msg("Invalid Authentication")]
    InvalidAuthentication
}