
use anchor_lang::prelude::error_code;


#[error_code]
pub enum ConfigError {
    #[msg("Fee cannot exceed 10%")]
    FeeTooHigh,
    #[msg("Fee to low")]
    FeeTooLow,
    #[msg("Duration is not realistic")]
    DurationNotRealistic,
    #[msg("This is exclusive to platform admins")]
    ExclusiveToAdmin,
    #[msg("Admins can't be authenticators")]
    AdminCannotbeAuthenticator,
    #[msg("Authenticator not in registory")]
    AuthenticatorNotInRegistry
}