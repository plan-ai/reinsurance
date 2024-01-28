use crate::{
    constant::DEFAULT_MINT_DECIMALS,
    error::InsuranceEnumError,
    event::StrategyAccepted,
    state::{Insurance, PremiumVault, ReInsuranceProposal, StrategyAccount, LP},
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct AcceptStrategy<'info> {
    pub strategy_accepter: Signer<'info>,
    #[account(
        mut,
        seeds = [
            lp.lp_creator.as_ref()
        ],
        bump=lp.bump
    )]
    pub lp: Account<'info, LP>,
    #[account(
        mint::decimals = DEFAULT_MINT_DECIMALS,
        seeds = [
            b"i_am_in_love",
            b"withacriminl",
            lp.key().as_ref()
        ],
        bump
    )]
    pub tokenised_mint: Account<'info, Mint>,
    #[account(
        seeds = [
            insurance.insurer.key().as_ref(),
            insurance.insurance_id.as_bytes()
        ],
        bump=insurance.bump,
        constraint = insurance.reinsured == true
    )]
    pub insurance: Account<'info, Insurance>,
    #[account(
        seeds = [
            lp.lp_creator.as_ref(),
            insurance.key().as_ref()
        ],
        bump=proposal.bump,
        constraint = proposal.proposal_accepted == true
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    #[account(
        seeds = [
            b"premium",
            insurance.key().as_ref(),
            proposal.key().as_ref()
        ],
        bump=premium_vault.bump
    )]
    pub premium_vault: Account<'info, PremiumVault>,
    #[account(
        mut,
        seeds = [
            b"strategy",
            proposed_strategy.strategy_id.as_bytes(),
            premium_vault.key().as_ref()
        ],
        bump=proposed_strategy.bump
    )]
    pub proposed_strategy: Account<'info, StrategyAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AcceptStrategy>) -> Result<()> {
    let proposed_strategy = &mut ctx.accounts.proposed_strategy;
    let tokenised_mint = &ctx.accounts.tokenised_mint;

    require!(
        tokenised_mint.supply <= proposed_strategy.vote * 2,
        InsuranceEnumError::NotEnoughVotes
    );

    proposed_strategy.strategy_accepted = true;

    emit!(StrategyAccepted {
        strategy: proposed_strategy.key()
    });

    Ok(())
}
