use crate::{
    constant::MONTH, error::InsuranceEnumError, event::ClaimDecisionReleased, state::Claim,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimDecision<'info> {
    pub decision_asker: Signer<'info>,
    #[account(mut)]
    pub claim: Account<'info, Claim>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimDecision>) -> Result<()> {
    let claim = &mut ctx.accounts.claim;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        current_time - claim.claim_voting_start.unwrap() > MONTH,
        InsuranceEnumError::DecisionNotYetReleased
    );

    if claim.vote_for > claim.vote_against {
        claim.accepted = Some(true);
    } else {
        claim.accepted = Some(false);
    };

    emit!(ClaimDecisionReleased {
        claim: claim.key(),
        decision: claim.accepted.unwrap()
    });

    Ok(())
}
