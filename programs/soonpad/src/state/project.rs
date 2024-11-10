use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Project {
    project_id: u64,
    #[max_len(100)] //TODO: ADD LOGIC FOR DYNAMIC SIZE
    wl: Vec<Pubkey>,
    hard_cap: u64,
    soft_cap: u64,
    raised: u64,
    project_bump: u8
}