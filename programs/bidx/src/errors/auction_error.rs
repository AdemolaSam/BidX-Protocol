use anchor_lang::prelude::error_code;


#[error_code]
pub enum AuctionError {
    #[msg("Start Date cannot be in the past")]
    StartDateIsBehind,

    #[msg("Reserved price cannot be lower than start price")]
    ReservedPriceTooLow,

    #[msg("End Date cannot be behind start date")]
    EndDateIsBehindStartDate,
}