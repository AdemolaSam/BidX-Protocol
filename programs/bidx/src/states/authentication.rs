use anchor_lang::prelude::*;

#[derive(Debug, Clone, InitSpace, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum AuthStatus {
    NotRequired,
    Pending,
    Verified,
    Rejected,
}

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
