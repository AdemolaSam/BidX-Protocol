use anchor_lang::prelude::error_code;

#[error_code]
pub enum BidError {
    #[msg("The Auction is Closed for Bidding")]
    BiddingClosed,

    #[msg("Amount is less than current highest bid")]
    BidTooLow,
}

