use crate::{
    constant::{DEFAULT_MINT_DECIMALS, WEEK},
    error::InsuranceEnumError,
    event::StrategyVoteRefunded,
    state::{StrategyAccount, StrategyVoteAccount, LP},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
#[derive(Accounts)]
pub struct RefundStrategyVote<'info> {
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
        mut,
        associated_token::mint = tokenised_mint,
        associated_token::authority = lp_token_owner,
    )]
    pub lp_token_owner_account: Account<'info, TokenAccount>,
    pub proposed_strategy: Account<'info, StrategyAccount>,
    #[account(
        mut,
        seeds = [
            proposed_strategy.key().as_ref(),
            lp_token_owner.key().as_ref()
        ],
        bump=proposed_strategy_vote_account.bump
    )]
    pub proposed_strategy_vote_account: Account<'info, StrategyVoteAccount>,
    #[account(
        mut,
        associated_token::mint = tokenised_mint,
        associated_token::authority = proposed_strategy_vote_account
    )]
    pub vote_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RefundStrategyVote>) -> Result<()> {
    let proposed_strategy = &ctx.accounts.proposed_strategy;
    let vote_token_account = &mut ctx.accounts.vote_token_account;
    let proposed_strategy_vote_account = &mut ctx.accounts.proposed_strategy_vote_account;
    let lp_token_owner_account = &mut ctx.accounts.lp_token_owner_account;
    let lp_token_owner = &ctx.accounts.lp_token_owner;
    let token_program = &ctx.accounts.token_program;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        current_time - proposed_strategy.voting_start.unwrap() > WEEK,
        InsuranceEnumError::RefundDeclined
    );

    let proposed_strategy_binding = proposed_strategy.key();
    let lp_token_owner_binding = lp_token_owner.key();
    let vote_account_seeds: &[&[&[u8]]] = &[&[
        proposed_strategy_binding.as_ref(),
        lp_token_owner_binding.as_ref(),
        &[proposed_strategy_vote_account.bump],
    ]];

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: vote_token_account.to_account_info(),
                to: lp_token_owner_account.to_account_info(),
                authority: proposed_strategy_vote_account.to_account_info(),
            },
            vote_account_seeds,
        ),
        vote_token_account.amount,
    )?;

    emit!(StrategyVoteRefunded {
        strategy: proposed_strategy.key(),
        refunded_to: lp_token_owner.key(),
        refund_amount: vote_token_account.amount
    });

    Ok(())
}
