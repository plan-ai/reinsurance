use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{Mint, Token, TokenAccount},
};
declare_id!("6T2GGYWJZAdNUSJMQ3xgcCUHPkiKNWoCQeym3HDtMxrw");

#[program]
pub mod strategy {
    use super::*;

    pub fn execute_strategy(_ctx: Context<ExecuteStrategyCPI>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteStrategyCPI<'info> {
    pub strategy_executor: Signer<'info>,
    #[account(mut)]
    pub premium_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = premium_vault
    )]
    pub premium_vault_token_account: Account<'info, TokenAccount>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    ///CHECK: program on which strategy is executed
    pub executor_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
