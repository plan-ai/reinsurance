use crate::{
    error::InsuranceEnumError,
    event::ReInsuranceProposalAccepted,
    state::{Insurance, ReInsuranceProposal},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AcceptReinsuranceProposal<'info> {
    pub insurance_creator: Signer<'info>,
    #[account(
        seeds = [
            insurance_creator.key().as_ref(),
            insurance.insurance_id.as_bytes()
        ],
        bump=insurance.bump
    )]
    pub insurance: Account<'info, Insurance>,
    #[account(
        mut,
        seeds = [
            proposal.lp_owner.key().as_ref(),
            insurance.key().as_ref()
        ],
        bump=proposal.bump
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AcceptReinsuranceProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let insurance = &ctx.accounts.insurance;

    require!(
        !insurance.reinsured,
        InsuranceEnumError::InsuranceReinsuredAlready
    );
    proposal.proposal_accepted = true;

    emit!(ReInsuranceProposalAccepted {
        reinsurance: proposal.key()
    });

    Ok(())
}
