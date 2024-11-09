use anchor_lang::prelude::*;

declare_id!("8SaP42D3JS89sDsrzthb3auioRvNU9mtGaMsDa8zGTtC");

#[program]
pub mod soonpad {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
