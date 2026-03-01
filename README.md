## INITIALIZE (SETUP PLATFORM)

This is the first step where the platform is setup, and inital authenticators are added.

_Requirements_

- MultiSig Admin Wallet
- Authenticators

_Settings_

- platform fee
- minimum auction duration (e.g. 1 hour)
- maximum auction duration (eg. 10 days)

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

## CREATE AUCTION

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
  ```

  ... to be continued

## REMOVE AUTHENTICATOR

** Warning - Check if authenticator is assigned to auctions before removal. Else, Auctions get stuck in "Pending" state unless Seller cancels and relist them **

## AUTHENTICATION

NOTE: authentication settlement is dependent on the success of the auction. for every successful Physical RWA auction that involves an authenticator, a fee set by the admin will be deducted at settlement and paid to the authenticator. THIS IS JUST FOR POC - WILL BE UPDATED LATER
