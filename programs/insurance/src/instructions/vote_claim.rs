use crate::{
    constant::MONTH,
    error::InsuranceEnumError,
    event::ClaimVoted,
    state::{Claim, ClaimVoteAccount},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(vote_amount:u64)]
pub struct VoteClaim<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub claim: Account<'info, Claim>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = voter,
        constraint = voter_token_account.amount>=vote_amount
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = voter,
        associated_token::mint = usdc_mint,
        associated_token::authority = claim
    )]
    pub claim_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = voter,
        space = 8 + ClaimVoteAccount::INIT_SPACE,
        seeds = [
            b"vote_account",
            claim.key().as_ref(),
            voter.key().as_ref()
        ],
        bump
    )]
    pub claim_vote_account: Account<'info, ClaimVoteAccount>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VoteClaim>, vote_amount: u64, vote_direction: bool) -> Result<()> {
    let claim_vote_account = &mut ctx.accounts.claim_vote_account;
    let claim = &mut ctx.accounts.claim;
    let voter = &mut ctx.accounts.voter;
    let claim_token_account = &mut ctx.accounts.claim_token_account;
    let voter_token_account = &mut ctx.accounts.voter_token_account;
    let token_program = &ctx.accounts.token_program;
    let current_time = Clock::get()?.unix_timestamp;

    match claim.claim_voting_start {
        None => claim.claim_voting_start = Some(current_time),
        Some(voting_start) => {
            require!(
                current_time - voting_start <= MONTH,
                InsuranceEnumError::ClaimVotingClosed
            )
        }
    };

    claim_vote_account.bump = ctx.bumps.claim_vote_account;
    claim_vote_account.vote_amount = vote_amount;
    claim_vote_account.voted_for = vote_direction;

    if vote_direction {
        claim.vote_for += vote_amount;
    } else {
        claim.vote_against += vote_amount
    };

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: voter_token_account.to_account_info(),
                to: claim_token_account.to_account_info(),
                authority: voter.to_account_info(),
            },
        ),
        vote_amount,
    )?;

    emit!(ClaimVoted {
        claim: claim.key(),
        voter: voter.key(),
        vote_amount: vote_amount
    });

    Ok(())
}
