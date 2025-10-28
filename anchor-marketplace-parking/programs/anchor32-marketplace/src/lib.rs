#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

declare_id!("6kuEXwmR9k4qAWmPcuxR2WisLkQfPJAQS55GQwJ6UmtN");

mod instructions;
mod state;
mod error;
mod constants;

pub use instructions::*;
pub use state::*;
pub use error::*;
pub use constants::*;

#[program]
pub mod anchor32_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u32) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    /*
       pub fn reserve(ctx: Context<Reserve>, start_time: i64, end_time: i64) -> Result<()> {
        ctx.accounts.reserve_listing(start_time, end_time)?;
        Ok(())
    }

    //pub fn update_reservation()

    pub fn sensor_change<'a>(ctx: Context<SwitchboardFeed>) -> Result<()> {
        //when driver arrives or leaves
        ctx.accounts.sensor_change()?;
        Ok(())
    }

    // {//driver scans QR code to confirm arrival and parking},
    //should also send alert to homeowner
    pub fn confirm_parking(ctx: Context<ConfirmParking>, sensor_id: String) -> Result<()> {
        ctx.accounts.confirm_parking(sensor_id)?;
        Ok(())
    }
    */

    
}

