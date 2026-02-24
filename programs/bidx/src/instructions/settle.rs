use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, TokenAccount, TransferChecked, close_account}};

use crate::{
    errors::AuctionError,
    events::AuctionSettled,
    states::{AssetType, Auction, AuctionStatus, Authentication, Bid, PlatformConfig}
};


#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct SettleAuction<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,

    ///CHECK : seller account
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,

    ///CHECK: authenticator account (if physical asset)
    #[account(mut)]
    pub authenticator: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"auction", seller.key().as_ref(), &nonce.to_le_bytes()],
        bump = auction.bump,
        has_one = seller
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        seeds = [b"bid", winner.key().as_ref(), auction.key().as_ref()],
        bump = bid.bump,
        has_one = auction
    )]
    pub bid: Account<'info, Bid>,

    #[account(
        mut,
        seeds = [b"authentication", auction.key().as_ref()],
        bump,
        constraint = authentication.auction == auction.key() @ AuctionAuthError::InvalidAuthentication,
    )]
    pub authentication: Option<Account<'info, Authentication>>,

    #[account(
        seeds = [b"config"],
        bump = platform_config.bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    //escrow vault - where winner bid is stored
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = bid,
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    //seller's token account (receives funds here)
    #[account(
        mut,
        constraint = seller_token_account.mint == token_mint.key() @ AuctionError::WrongToken,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    //Platform treasury
    #[account(
        mut,
        constraint = treasury.key() ==platform_config.treasury_usdc || treasury.key() == platform_config.treasury_sol @AuctionError::InvalidTreasury
    )]
    pub treasury: Account<'info, TokenAccount>,

    //Authenticators token account (if physical asset)
    #[account(
        mut,
        constraint = authenticator_token_account.mint == token_mint.key() @AuctionError::WrongToken,
    )]
    pub authenticator_token_account: Option<Account<'info, TokenAccount>>,

    //NFT Accounts
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = auction
    )]
    pub item_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = winner,
    )]
    pub winner_nft_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> SettleAuction <'info> {
    pub fn settle_auction(&mut self) -> Result<()> {
        let auction = &mut self.auction;
        let bid = &self.bid;

        //validations
        require!(
            auction.auction_status == AuctionStatus::Ended,
            AuctionError::AuctionNotEnded
        );

        require!(
            self.winner.key() == auction.highest_bidder,
            AuctionError::NotWinner
        );

        require!(
            auction.highest_bid >= auction.reserved_price,
            AuctionError::ReserveNotMet
        );

        require!(
            bid.is_active,
            AuctionError::BidNotActive
        );

        let winning_bid = auction.highest_bid;
        let plaform_fee_bps = self.platform_config.platform_fee_bps;

        //calculate fees
        let platform_fee = (winning_bid as u128)
            .checked_mul(plaform_fee_bps as u128)
            .unwrap()
            .checked_div(10_000)
            .unwrap() as u64;

        // seller amount
        let mut seller_amount = winning_bid - platform_fee;
        //auth fee
        let mut auth_fee: u64 = 0;

        //if physical asset; deduct auth fee
        if self.auction.asset_type == AssetType::PhysicalRWA {
            let auth_fee_bps = self.platform_config.auth_fee_bps;
            auth_fee = (winning_bid as u128)
                .checked_mul(auth_fee_bps as u128)
                .unwrap()
                .checked_div(10_000)
                .unwrap() as u64;

            seller_amount = seller_amount - auth_fee;
        }

        //PDA SEEDS
        let auction_seeds = &[
            b"auction",
            self.seller.key().as_ref(),
            &nonce.to_le_bytes(),
            [&auction.bump]
        ];

        let auction_signer_seeds = &[&auction_seeds[..]];

       let bid_seeds = &[
            b"bid",
            self.winner.key().as_ref(),
            auction.key().as_ref(),
            &[bid.bump],
        ];

        let bid_signer_seeds = &[&bid_seeds[..]]; //bid_signer

        // Settle Seller
        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked{
                    from: self.escrow_vault.to_account_info(),
                    to: self.seller_token_account.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                    authority: self.bid.to_account_info(),
                },
                bid_signer_seeds
            ),
            seller_amount,
            self.token_mint.decimals
        );

        //settle platform (pay fee to treasury)
        transfer_checked(
            CpiContext::new_with_signer(
            self.token_program.to_account_info(),
                TransferChecked{
                    from: self.escrow_vault.to_account_info(),
                    to: self.treasury.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                    authority: self.bid.to_account_info(),
                },
                bid_signer_seeds
            ),
            platform_fee,
            self.token_mint.decimals
        );

        // settle authenticator (if Physical Real World Asset)
        if auction.asset_type == AssetType::PhysicalRWA {
            if let Some(auth_token_account) = &self.authenticator_token_account {

                transfer_checked(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        TransferChecked{
                            from: self.escrow_vault.to_account_info(),
                            to: auth_token_account.to_account_info(),
                            mint: self.token_mint.to_account_info(),
                            authority: self.bid.to_account_info()
                        },
                        bid_signer_seeds
                    ),
                    auth_fee,
                    self.token_mint.decimals
                )
            }

            //update auth fee as paid
            if let Some(auth) = &mut self.authentication {
                auth.fee_amount = auth_fee;
                auth.fee_paid = true;
            }
        };

        // Transfer NFT to winner
        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked{
                    from: self.item_vault.to_account_info(),
                    to: self.winner_nft_account.to_account_info(),
                    mint: self.nft_mint.to_account_info(),
                    authority: self.auction.to_account_info()
                },
                auction_signer_seeds
            ),
            1,
            self.nft_mint.decimals,
        );

        //close escrow vault for rent reclaim
        close_acct_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.escrow_vault.to_account_info(),
                destination: self.winner.to_account_info(),
                authority: self.bid.to_account_info()
            }, 
            bid_signer_seeds
        );
        close_account(close_acct_ctx);

        //update auction status
        auction.auction_status = AuctionStatus::Settled;

        emit!(AuctionSettled {
            auction: auction.key(),
            winner: self.winner.key(),
            final_price: winning_bid,
            platform_fee,
            auth_fee,
            seller_amount,
        });

        

        Ok(())

    }
}

