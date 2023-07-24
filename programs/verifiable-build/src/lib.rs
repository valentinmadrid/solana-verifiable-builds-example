use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("AD62W7aAQETg11ajkadk57DYfPTWPzKLiXuLZVB4eQaw");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    recipient: SystemAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}
