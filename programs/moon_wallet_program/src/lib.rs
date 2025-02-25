use anchor_lang::prelude::*;

declare_id!("GnTr7FsPPFxQydLUzoQvVwL64TBFwU9MznpTqzJHM7Bc");

#[program]
pub mod moon_wallet_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
