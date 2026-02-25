use anchor_lang::prelude::*;

use crate::events::PlatformConfigUpdated;
use crate::states::PlatformConfig;
use crate::errors::ConfigError;

#[derive(Accounts)]
pub struct UpdatePlatformConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config".as_ref()],
        bump,
        has_one = admin
    )]
    pub platform_config: Account<'info, PlatformConfig>,
}

impl <'info> UpdatePlatformConfig<'info> {
    pub fn update_platform_config(
        &mut self,
        platform_fee_bps: Option<u16>,
        auth_fee_bps: Option<u16>,
        min_auction_duration: Option<i64>,
        max_auction_duration: Option<i64>,
    ) -> Result<()>{
        let new_fee_bps = platform_fee_bps.unwrap_or(self.platform_config.platform_fee_bps);
        let new_auth_fee_bps = auth_fee_bps.unwrap_or(self.platform_config.auth_fee_bps);
        let new_min_duration = min_auction_duration.unwrap_or(self.platform_config.min_auction_duration);
        let new_max_duration = max_auction_duration.unwrap_or(self.platform_config.max_auction_duration);

        require!(
            new_fee_bps > 0,
            ConfigError::FeeTooLow
        );

        require!(
            new_auth_fee_bps > 0,
            ConfigError::FeeTooLow
        );

        require!(
            new_min_duration > 0,
            ConfigError::DurationNotRealistic
        );

        require!(
            max_auction_duration > min_auction_duration,
            ConfigError::DurationNotRealistic
        );

        let mut updated_fields: Vec<String> = vec![];

        if new_fee_bps != self.platform_config.platform_fee_bps {
            self.platform_config.platform_fee_bps = new_fee_bps;
            updated_fields.push(String::from("platform_fee_bps"));
        };
        if new_auth_fee_bps != self.platform_config.auth_fee_bps {
            self.platform_config.auth_fee_bps = new_auth_fee_bps;
            updated_fields.push(String::from("auth_fee_bps"))
        };
        if new_min_duration != self.platform_config.min_auction_duration {
            self.platform_config.min_auction_duration = new_min_duration;
            updated_fields.push(String::from("min_auction_duration"))
        };
        if new_max_duration != self.platform_config.max_auction_duration {
            self.platform_config.max_auction_duration = new_max_duration;
            updated_fields.push(String::from("max_auction_duration"));
        };

        emit!(
            PlatformConfigUpdated {
                fields: updated_fields,
                timestamp: Clock::get()?.unix_timestamp,
            }
        );

        Ok(())
    }
}