//TODO
// calculate how much each person gets
// invested_amount * price_per_token

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{Project, User};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"project", project.project_id.as_bytes()],
        bump = project.project_bump
    )]
    pub project: Account<'info, Project>,
    #[account(
        seeds = [b"user", project.key().as_ref(), signer.key().as_ref()],
        bump = user.user_bump
    )]
    pub user: Account<'info, User>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub signer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = project
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
impl<'info> Claim<'info> {
    pub fn claim(&mut self, bumps: ClaimBumps) -> Result<()> {
        let accounts = TransferChecked {
            from: self.token_vault.to_account_info(),
            to: self.signer_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.project.to_account_info(),
        };
        let binding = [self.project.project_bump];
        let signer_seeds = &[&[b"project", self.project.project_id.as_bytes(), &binding][..]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        let amount = self
            .user
            .invested_amount
            .checked_mul(self.project.price_per_token)
            .unwrap();
        transfer_checked(ctx, amount, self.mint.decimals)?;
        Ok(())
    }
}
