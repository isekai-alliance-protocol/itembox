use anchor_lang::prelude::*;

declare_id!("4nguiyc8un36HUSaS14K7yau91JPyed4oEadt5hVR7BT");

#[program]
pub mod itembox {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
