use anchor_lang::prelude::error_code;


#[error_code]
pub enum AuctionError {
    #[msg("Start Date cannot be in the past")]
    StartDateIsBehind,

    #[msg("Reserved price cannot be lower than start price")]
    ReservedPriceTooLow,

    #[msg("End Date cannot be behind start date")]
    EndDateIsBehindStartDate,

    #[msg("Wrong Token")]
    WrongToken,

    #[msg("Invalid treasury token")]
    InvalidTreasury,

    #[msg("Auction not ended")]
    AuctionNotEnded,

    #[msg("Not Winner")]
    NotWinner,

    #[msg("Reserve not met")]
    ReserveNotMet,

    #[msg("Bid not active")]
    BidNotActive,
}