use anchor_lang::prelude::*;


#[event]
pub struct PlatformInitialized {
    pub message: String,
    pub admin: Pubkey,
    pub total_authenticators: u64
}

#[event]
pub struct PlatformConfigUpdated {
    pub fields: Vec<String>,
    pub timestamp: i64,
}

#[event]
pub struct PlatformPaused {
    pub reason: String,
    pub timestamp: i64,
}

#[event]
pub struct AuthenticatorsAddedToPlatform {
    pub total_added: u64,
    pub timestamp: i64,
}

#[event]
pub struct AuthenticatorRemovedFromPlatform {
    pub authenticator: Pubkey,
    pub timestamp: i64
}

#[event]
pub struct PlatformPauseToggled {
    pub is_paused: bool,
    pub timestamp: i64,
}