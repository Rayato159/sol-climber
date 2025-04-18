use anchor_lang::prelude::*;

declare_id!("4J6zJvMRSdvTeodiDg5pmQqzLrSvNzA84LCQLLoHxXMe");

#[program]
pub mod sol_climber_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
