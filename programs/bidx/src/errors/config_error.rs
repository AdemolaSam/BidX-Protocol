
use anchor_lang::prelude::error_code;


#[error_code]
pub enum ConfigError {
    #[msg("Fee cannot exceed 10%")]
    FeeTooHigh
}