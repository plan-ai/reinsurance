use crate::{
    constant::{DEFAULT_MINT_DECIMALS, WEEK},
    error::InsuranceEnumError,
    event::StrategyVoted,
    state::{
        Insurance, PremiumVault, ReInsuranceProposal, StrategyAccount, StrategyVoteAccount, LP,
    },
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
#[derive(Accounts)]
#[instruction(vote_amount:u64)]
pub struct VoteStrategy<'info> {
    #[account(mut)]
    pub lp_token_owner: Signer<'info>,
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
        mut,
        associated_token::mint = tokenised_mint,
        associated_token::authority = lp_token_owner,
        constraint = lp_token_owner_account.amount >= vote_amount @InsuranceEnumError::InsufficientVotingPower
    )]
    pub lp_token_owner_account: Account<'info, TokenAccount>,
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
    #[account(
        init_if_needed,
        space = 8+ StrategyVoteAccount::INIT_SPACE,
        payer = lp_token_owner,
        seeds = [
            proposed_strategy.key().as_ref(),
            lp_token_owner.key().as_ref()
        ],
        bump
    )]
    pub proposed_strategy_vote_account: Account<'info, StrategyVoteAccount>,
    #[account(
        init_if_needed,
        payer = lp_token_owner,
        associated_token::mint = tokenised_mint,
        associated_token::authority = proposed_strategy_vote_account
    )]
    pub vote_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VoteStrategy>, vote_amount: u64) -> Result<()> {
    let proposed_strategy = &mut ctx.accounts.proposed_strategy;
    let proposed_strategy_vote_account = &mut ctx.accounts.proposed_strategy_vote_account;
    let vote_token_account = &mut ctx.accounts.vote_token_account;
    let lp_token_owner_account = &mut ctx.accounts.lp_token_owner_account;
    let lp_token_owner = &mut ctx.accounts.lp_token_owner;
    let token_program = &ctx.accounts.token_program;
    let current_time = Clock::get()?.unix_timestamp;

    match proposed_strategy.voting_start {
        None => {
            proposed_strategy.voting_start = Some(current_time);
        }
        Some(voting_start) => {
            require!(
                current_time - voting_start <= WEEK,
                InsuranceEnumError::VotingOnStrategyClosed
            );
        }
    };
    // add to vote filed
    proposed_strategy.vote += vote_amount;
    // sets bump of vote account for future reference
    proposed_strategy_vote_account.bump = ctx.bumps.proposed_strategy_vote_account;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: lp_token_owner_account.to_account_info(),
                to: vote_token_account.to_account_info(),
                authority: lp_token_owner.to_account_info(),
            },
        ),
        vote_amount,
    )?;

    emit!(StrategyVoted {
        strategy: proposed_strategy.key(),
        voter: lp_token_owner.key(),
        vote_amount: vote_amount,
    });

    Ok(())
}
