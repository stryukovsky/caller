use anchor_lang::prelude::*;
use callee::cpi::accounts::SetTime;
use callee::cpi::set_time;
use callee::ClockData;
use callee::program::Callee;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod caller {
    use super::*;

    pub fn call(ctx: Context<Call>) -> Result<()> {
        let accounts = SetTime{
            authority: ctx.accounts.authority.to_account_info(),
            clock_data: ctx.accounts.clock_data.to_account_info(),
        };
        let program = ctx.accounts.callee_program.to_account_info();
        let context = CpiContext::new(program, accounts);
        set_time(context)
    }
}

#[derive(Accounts)]
pub struct Call<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub clock_data: Account<'info, ClockData>,

    pub callee_program: Program<'info, Callee>,

}
