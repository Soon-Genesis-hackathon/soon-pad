use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub authority: Pubkey,
    pub invested_amount: u64,
    #[max_len(20)]
    pub project_id: String,
    pub user_bump: u8,
}
