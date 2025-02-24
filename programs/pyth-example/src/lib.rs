use anchor_lang::prelude::*;

declare_id!("n3t6sxhffRkPNs8F7pyWMjwrSEnWmRScFBVwofpYNWj");

#[program]
pub mod pyth_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
