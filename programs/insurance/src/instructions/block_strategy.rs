use crate::{constant::AUTHORIZED_PUBLIC_KEY, event::StrategyBlocked, state::StrategyAccount};
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct BlockStrategy<'info> {
    #[account(address=AUTHORIZED_PUBLIC_KEY)]
    pub authorised_address: Signer<'info>,
    #[account(
        mut,
        constraint = proposed_strategy.strategy_accepted == true
    )]
    pub proposed_strategy: Account<'info, StrategyAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BlockStrategy>) -> Result<()> {
    let proposed_strategy = &mut ctx.accounts.proposed_strategy;

    proposed_strategy.strategy_blocked = true;

    emit!(StrategyBlocked {
        strategy: proposed_strategy.key()
    });
    Ok(())
}
