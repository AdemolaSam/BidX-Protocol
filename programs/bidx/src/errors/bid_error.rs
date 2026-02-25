use anchor_lang::prelude::error_code;

#[error_code]
pub enum BidError {
    #[msg("Auction not available")]
    AuctionNotAvailable,

    #[msg("The Auction is Closed for Bidding")]
    BiddingClosed,

    #[msg("Amount is less than current highest bid")]
    BidTooLow,

    #[msg("Wrong token type")]
    WrongToken,

    #[msg("Withdrawal disabled. You are currently winnning")]
    StillWinning,
}

