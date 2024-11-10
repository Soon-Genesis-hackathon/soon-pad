pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;

pub use state::*;
pub use instructions::*;

declare_id!("8SaP42D3JS89sDsrzthb3auioRvNU9mtGaMsDa8zGTtC");
#[program]
pub mod soonpad {

    use super::*;

    pub fn create_project(ctx: Context<CreateProject>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    // pub fn buy(ctx: Context<Buy>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
    // pub fn claim(ctx: Context<Claim>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
}
