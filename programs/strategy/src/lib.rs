use anchor_lang::prelude::*;

declare_id!("6T2GGYWJZAdNUSJMQ3xgcCUHPkiKNWoCQeym3HDtMxrw");

#[program]
pub mod strategy {
    use super::*;
    
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
}

#[account]
pub struct Data {
    pub data: u64,
}
