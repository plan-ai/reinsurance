use crate::{
    error::InsuranceEnumError,
    event::ClaimRaised,
    state::{Claim, Insurance, ReInsuranceProposal, LP},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(claim_id:String,claim_amount:u64)]
pub struct RaiseClaim<'info> {
    #[account(mut)]
    pub insurance_creator: Signer<'info>,
    #[account(
        seeds = [
            insurance_creator.key().as_ref(),
            insurance.insurance_id.as_bytes()
        ],
        bump=insurance.bump,
        constraint = insurance.coverage >= claim_amount @InsuranceEnumError::ClaimTooHigh
    )]
    pub insurance: Account<'info, Insurance>,
    #[account(
        seeds = [
            proposal.lp_owner.as_ref()
        ],
        bump=lp.bump
    )]
    pub lp: Account<'info, LP>,
    #[account(
        mut,
        seeds = [
            proposal.lp_owner.as_ref(),
            insurance.key().as_ref()
        ],
        bump=proposal.bump,
        constraint = proposal.proposal_accepted == true,
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    #[account(
        init,
        payer = insurance_creator,
        space = 8+Claim::INIT_SPACE,
        seeds = [
            b"claim",
            proposal.key().as_ref(),
            claim_id.as_bytes()
        ],
        bump
    )]
    pub claim: Account<'info, Claim>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RaiseClaim>,
    claim_id: String,
    claim_amount: u64,
    claim_metadata_link: String,
) -> Result<()> {
    let claim = &mut ctx.accounts.claim;
    let proposal = &ctx.accounts.proposal;

    claim.bump = ctx.bumps.claim;
    claim.reinsurance = proposal.key();
    claim.claim_id = claim_id;
    claim.claim_amount = claim_amount;
    claim.claim_metadata_link = claim_metadata_link.clone();

    emit!(ClaimRaised {
        reinsurance: proposal.key(),
        claim: claim.key(),
        claim_amount: claim_amount,
        claim_metadata_link: claim_metadata_link
    });

    Ok(())
}
