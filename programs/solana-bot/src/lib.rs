use anchor_lang::prelude::*;

declare_id!("5QySMwEVWLGXj1zhGo2KQSgXUJRvEEnRGU911KZofFcr");

#[program]
pub mod solana_bot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
