use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{Project, User};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"project", project.project_id.as_bytes()],
        bump = project.project_bump
    )]
    pub project: Account<'info, Project>,
    #[account(
        mut,
        seeds = [b"projectVault", project.project_id.as_bytes()],
        bump
    )]
    pub project_vault: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"user", signer.key().as_ref()],
        space = User::INIT_SPACE,
        bump
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
impl<'info> Buy<'info> {
    pub fn buy(&mut self, amount: u64, bumps: BuyBumps) -> Result<()> {
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.project_vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(ctx, amount)?;
        self.user.set_inner(User {
            authority: self.signer.key(),
            invested_amount: self.user.invested_amount.checked_add(amount).unwrap(),
            project_id: self.project.project_id.clone(),
            user_bump: bumps.user,
        });
        Ok(())
    }
}
