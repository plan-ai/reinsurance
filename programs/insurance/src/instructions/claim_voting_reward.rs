use crate::{
    error::InsuranceEnumError,
    event::ClaimRewardReleased,
    state::{Claim, ClaimVoteAccount},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct ClaimVotingReward<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub claim: Account<'info, Claim>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = voter,
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = claim
    )]
    pub claim_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            b"vote_account",
            claim.key().as_ref(),
            voter.key().as_ref()
        ],
        bump=claim_vote_account.bump
    )]
    pub claim_vote_account: Account<'info, ClaimVoteAccount>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimVotingReward>, reward_amount: u64) -> Result<()> {
    let claim = &mut ctx.accounts.claim;
    let claim_vote_account = &mut ctx.accounts.claim_vote_account;
    let claim_token_account = &mut ctx.accounts.claim_token_account;
    let voter_token_account = &mut ctx.accounts.voter_token_account;
    let token_program = &ctx.accounts.token_program;

    require!(
        claim_vote_account.voted_for == claim.accepted.unwrap(),
        InsuranceEnumError::ClaimVoteDidNotWin
    );

    if claim_vote_account.voted_for {
        require!(
            reward_amount * claim.vote_for == claim_vote_account.vote_amount * claim.vote_against,
            InsuranceEnumError::IncorrectRewardAmount
        );
    } else {
        require!(
            reward_amount * claim.vote_against == claim_vote_account.vote_amount * claim.vote_for,
            InsuranceEnumError::IncorrectRewardAmount
        );
    }

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"claim",
        claim.reinsurance.as_ref(),
        claim.claim_id.as_bytes(),
        &[claim.bump],
    ]];

    let transfer_amount = claim_vote_account.vote_amount + reward_amount;

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: claim_token_account.to_account_info(),
                to: voter_token_account.to_account_info(),
                authority: claim.to_account_info(),
            },
            signer_seeds,
        ),
        transfer_amount,
    )?;

    claim_vote_account.vote_amount = 0;
    emit!(ClaimRewardReleased {
        claim: claim.key(),
        reward_amount: reward_amount
    });

    Ok(())
}
