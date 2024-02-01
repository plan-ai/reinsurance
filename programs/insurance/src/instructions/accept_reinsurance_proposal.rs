use crate::{
    error::InsuranceEnumError,
    event::ReInsuranceProposalAccepted,
    state::{Insurance, ReInsuranceProposal, LP},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AcceptReinsuranceProposal<'info> {
    pub insurance_creator: Signer<'info>,
    #[account(
        mut,
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
        bump=proposal.bump
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AcceptReinsuranceProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let insurance = &mut ctx.accounts.insurance;
    let lp = &mut ctx.accounts.lp;
    let current_time = Clock::get()?.unix_timestamp;

    lp.total_securitized += insurance.coverage;
    if proposal.proposed_undercollaterization > lp.max_undercollaterization_promised {
        lp.max_undercollaterization_promised = proposal.proposed_undercollaterization;
    }
    lp.undercollaterization_promised
        .push(proposal.proposed_undercollaterization);

    require!(
        lp.total_assets * 1000 >= lp.max_undercollaterization_promised * lp.total_securitized,
        InsuranceEnumError::CanNotFullFillUnderCollateralizationDemands
    );
    require!(
        !insurance.reinsured,
        InsuranceEnumError::InsuranceReinsuredAlready
    );
    proposal.proposal_accepted = true;
    insurance.reinsured = true;
    insurance.premium_due = Some(current_time);

    emit!(ReInsuranceProposalAccepted {
        reinsurance: proposal.key()
    });

    Ok(())
}
