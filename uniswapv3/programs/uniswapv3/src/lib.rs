use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod events;
pub mod errors;
pub mod utils;

declare_id!("38vL7QvvuDKksoSjCAu7HUqXudvNoNSfRhB7LiYSED7e");

#[program]
pub mod uniswapv3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
