use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("8evVxMNMoHX2gdGVSLfHR4UwRUCdYC28erfpUAYaPiEn");

#[program]
pub mod insurance {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
