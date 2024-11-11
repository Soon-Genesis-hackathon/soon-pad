use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::Project;

#[derive(Accounts)]
#[instruction(project_id: String)]
pub struct CreateProject<'info> {
    // TODO: ADD CHECKS FOR ADMIN
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"project", project_id.as_bytes()],
        space = Project::INIT_SPACE,
        bump
    )]
    pub project: Account<'info, Project>,
    #[account(
        mut,
        seeds = [b"projectVault", project.project_id.as_bytes()],
        bump
    )]
    pub project_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> CreateProject<'info> {
    pub fn create_project(
        &mut self,
        project_id: String,
        project_name: String,
        project_authority: Pubkey,
        price_per_token: u64,
        hard_cap: u64,
        soft_cap: u64,
        raised: u64,
        bumps: &CreateProjectBumps,
    ) -> Result<()> {
        self.project.set_inner(Project {
            project_id: project_id.clone(),
            project_name,
            project_authority,
            price_per_token,
            project_status: 0,
            wl: vec![Pubkey::new_unique()],
            hard_cap,
            soft_cap,
            raised,
            project_bump: bumps.project,
        });
        msg!("Created a new project with project id {}", project_id);
        let rent = Rent::get()?;
        let amount = rent.minimum_balance(0);
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.project_vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)?;
        msg!("Created a project vault for the project");
        Ok(())
    }
}
