use anchor_lang::prelude::*;

declare_id!("BuKceHeqNj5q5jLhD3Zn5QRTrNozTNtq4ShKmTUfa8Bj");

#[program]
pub mod tensorflowsolana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
