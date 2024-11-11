use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Project {
    #[max_len(20)]
    pub project_id: String,
    #[max_len(40)]
    pub project_name: String,
    pub project_authority: Pubkey,
    pub price_per_token: u64,
    #[max_len(5)] //TODO: ADD LOGIC FOR DYNAMIC SIZE
    pub wl: Vec<Pubkey>,
    pub hard_cap: u64,
    pub soft_cap: u64,
    pub raised: u64,
    pub project_status: u8, // 0 - Whitelisting, 1 - Sale, 2 - Distribution, 3 - Ended
    pub project_bump: u8,
}
