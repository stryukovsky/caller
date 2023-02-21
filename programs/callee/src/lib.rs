use anchor_lang::prelude::*;

declare_id!("Qk6aFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod callee {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.clock_data.timestamp = 0;
        Ok(())
    }

    pub fn set_time(ctx: Context<SetTime>) -> Result<()> {
        ctx.accounts.clock_data.timestamp = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = 1000)]
    pub clock_data: Account<'info, ClockData>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTime<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub clock_data: Account<'info, ClockData>,
}

#[account]
pub struct ClockData {
    pub timestamp: i64,
}
