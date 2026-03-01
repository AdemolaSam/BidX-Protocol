use anchor_lang::prelude::*;

use crate::{
    errors::ConfigError, 
    events::PlatformPauseToggled, 
    states::PlatformConfig
};

#[derive(Accounts)]
pub struct TogglePause <'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config", platform_config.admin.as_ref()],
        bump = platform_config.bump,
        has_one = admin @ ConfigError::ExclusiveToAdmin,
    )]
    pub platform_config: Account<'info, PlatformConfig>
}


impl<'info> TogglePause <'info> {
    pub fn toggle_pause (&mut self) -> Result<()> {
        let paused = self.platform_config.is_paused;
        self.platform_config.is_paused = !paused;
        
        emit!(
            PlatformPauseToggled {
                is_paused: self.platform_config.is_paused,
                timestamp: Clock::get()?.unix_timestamp
            }
        );

        Ok(())

    }
}
