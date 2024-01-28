use crate::{
    event::StrategyProposed,
    state::{PremiumVault, StrategyAccount},
    strategy_program_interface::StrategyInterface,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(strategy_id:String)]
pub struct ProposeStrategy<'info> {
    #[account(mut)]
    pub strategy_proposer: Signer<'info>,
    pub premium_vault: Account<'info, PremiumVault>,
    #[account(
        init,
        payer = strategy_proposer,
        space = 8+StrategyAccount::INIT_SPACE,
        seeds = [
            b"strategy",
            strategy_id.as_bytes(),
            premium_vault.key().as_ref()
        ],
        bump
    )]
    pub proposed_strategy: Account<'info, StrategyAccount>,
    pub strategy_program: Interface<'info, StrategyInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ProposeStrategy>,
    strategy_id: String,
    max_spending_power: u64,
) -> Result<()> {
    let strategy_program = &ctx.accounts.strategy_program;
    let premium_vault = &ctx.accounts.premium_vault;
    let proposed_strategy = &mut ctx.accounts.proposed_strategy;

    proposed_strategy.bump = ctx.bumps.proposed_strategy;
    proposed_strategy.strategy_program = strategy_program.key();
    proposed_strategy.max_spending_power = max_spending_power;
    proposed_strategy.strategy_id = strategy_id.clone();
    proposed_strategy.premium_vault = premium_vault.key();
    proposed_strategy.vote = 0;
    proposed_strategy.voting_start = None;
    proposed_strategy.strategy_accepted = false;

    emit!(StrategyProposed {
        strategy: proposed_strategy.key(),
        max_spending_power: max_spending_power,
        premium_vault: premium_vault.key(),
        strategy_id: strategy_id
    });

    Ok(())
}
