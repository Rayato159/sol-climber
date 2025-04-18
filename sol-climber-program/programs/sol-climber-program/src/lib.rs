use anchor_lang::prelude::*;

declare_id!("2yhka4N7fLLm5sAXDAAinCL7g2EiVsMNrFRPk2c7VfjH");

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
