# PROJECT OVERVIEW

BidX is an open-source auction protocol on Solana for high-value digital and physical assets, starting with NFT collectibles and luxury watches. Unlike traditional platforms (eBay, Heritage Auctions, StockX), BidX requires bidders to lock funds in escrow before bidding—eliminating fake bids, payment defaults, and non-serious participants. Every bid and authentication record is recorded on-chain, providing cryptographic proof of auction fairness that buyers can independently verify. Settlement is instant: when auctions close, funds transfer to sellers and assets transfer to winners in seconds, not weeks. We're targeting NFT traders (immediate traction) and cross-border luxury collectors holding stablecoins who are currently locked out of traditional platforms by KYC barriers.

Read the full project proposal here: [BidX-Protocol full proposal](https://docs.google.com/document/d/1aXTD0X6sAnh9L7fHNwXLEFFMvSHGfe6dRo3Q-SViKsQ/edit?usp=sharing)

## RUNNING

**REQUIREMENTS**
You need to have the following installed [Check Here for full Rust/Solana/Anchor Installation](https://www.anchor-lang.com/docs/installation)

- Rust
- Solana
- Anchor
- NodeJs
- Yarn

Clone or fork this repo
Then run:

```bash
  anchor build
  yarn install
```

## TESTING

Localnet (full suite):

```bash
anchor test
```

Devnet (full suite; platform PDAs are derived from the admin wallet):

```bash
anchor test --provider.cluster devnet
```

Note: On devnet, tests fund new accounts by transferring SOL from your `ANCHOR_WALLET`.
Make sure that wallet is funded before running tests. The test helpers default to
funding 0.05 SOL per new account, and the admin gets 0.5 SOL, so a full devnet run
stays under ~3 SOL total.

Devnet timing: auctions use a 20s start delay and 60s duration to avoid clock skew,
so the settle/withdraw suite takes longer on devnet.

## BIDX Protocol's Architectural Diagram

Summarized Flow

![Summarized Flow Chart](./Assets//BidX-Summarized%20Arch%20Diagram.drawio.png)

Full

![Full](./Assets//BidX%20Architectural%20Diagram-2026-02-08-030655.png)

See the different component at: [Architectural Diagram](https://docs.google.com/document/d/1seagNHfNQQNR2gh0QuAQ1Ie_4g7tpYkMLmbrSVc-G-4/edit?usp=sharing)

## CORE INSTRUCTIONS

### initialize (SETUP PLATFORM)

This is the first step where the platform is setup, and inital authenticators are added.

_Requirements_

- Admin Wallet
- Authenticators (Physical Item Validators - They ensure item being auctioned meets the specified standards and stated conditions by the seller)

_Settings_

- platform fee
- authenticator fee
- minimum auction duration (e.g. 1 hour)
- maximum auction duration (eg. 10 days)

Authenticators Registry

```Rust
  #[account]
  #[derive(InitSpace)]
  pub struct AuthenticatorsRegistry {
      pub admin: Pubkey,
      #[max_len(100)]
      pub authenticators: Vec<Pubkey>,
      pub next_index: u64, // for programmatically assigning authenticators to auctions that require physical verification and approval
      pub bump: u8,
  }

```

### create_auction

This is the process of listing auctions and making the available to the public after approval

_Requirements_

- Seller with Assets: Digital NFT or Physical Real world assets (e.g. wristwatches)
- If physical asset, authentication is required. An authentication record is created and an authenticator is assigned programmatically using "round robin". See the authentication account:

  ```Rust
  #[account]
  #[derive(InitSpace)]
  pub struct Authentication {
      pub auction: Pubkey,
      pub auth_status: AuthStatus,
      pub authenticator: Pubkey,
      pub seller: Pubkey,
      #[max_len(300)]
      pub metadata_hash: String, // IPFS hash containig item documentation from seller
      #[max_len(300)]
      pub report_hash: String, // IPFS hash containing item verification report from seller
      pub uploaded_at: i64, // report hash upload timestamp
      pub verified_at: i64,
      pub fee_amount: u64,
      pub fee_paid: bool,
      pub bump: u8,
  }

  AUTHENTICATION STATUS
  #[derive(Debug, Clone, InitSpace, AnchorSerialize, AnchorDeserialize, PartialEq)]
  pub enum AuthStatus {
    NotRequired, // Digital assets does not require authentication
    Pending,
    Verified,
    Rejected,
  }
  ```

### register_authenticators & remove_authenticator

Admin can add authenticators to the platform and can remove authenticators

** Warning - Check if authenticator is assigned to auctions before removal. Else, Auctions get stuck in "Pending" state unless Seller cancels and relist them **

### settle

After auction expiry time, if reserved price is not met the locked NFT will be withdrawable by the seller. Otherwise, settlement instruction is called.

- The NFT is immediately released to the winner through
- The seller gets payed from the locked escrow
- Platform fee is deposited to platform's treasury
- Auhtenticator's fee is paid (if Physical Real World Asset)

### withdraw_bid

None Winning bids makes fund availble for withdrawals into bidder account

### update_platfom_config

Admin can update

- platform fee
- authentication fee
- min aucttion start time
- max auction start time

### toggle_pause_platform

Admin can pause the platorm if a critical error/issue is discovered in the platform. They can unpause it as well

### upload_auth_report

Authenticators can upload report hash containing findings on a Physical Asset they have been assigned to verify

### attest_authentication

Authenticators can approve or decline a Physical RWA if their findings about the asset is not satisfactory (this is only possible after they have uploaded report about the asset)
