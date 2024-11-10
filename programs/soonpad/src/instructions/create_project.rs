use anchor_lang::prelude::*;

use crate::Project;

#[derive(Accounts)]
pub struct CreateProject<'info> {
    // TODO: ADD CHECKS FOR ADMIN
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [],
        space = Project::INIT_SPACE,
        bump
    )]
    pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>
}
impl<'info> CreateProject<'info> {

}