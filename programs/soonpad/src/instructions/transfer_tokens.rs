// Tokens created from client side. Transfer the required tokens to the vault
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

use crate::Project;

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)] // Only admin can sign this transaction
    pub signer: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"project", project.project_id.as_bytes()],
        bump = project.project_bump
    )]
    pub project: Account<'info, Project>,
    #[account(
        init,
        associated_token::mint = mint,
        associated_token::authority = project,
        payer = signer
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> TransferTokens<'info> {
    pub fn transfer_tokens_to_vault(&mut self) -> Result<()> {
        // ADD A CHECK
        let amount = self.project.price_per_token.checked_mul(self.project.raised).unwrap();
        let accounts = TransferChecked {
            from: self.signer.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.token_vault.to_account_info(),
            authority: self.project.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer_checked(ctx, amount, self.mint.decimals)?;
        Ok(())
    }
}