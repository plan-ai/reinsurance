use crate::{
    error::InsuranceEnumError,
    event::ReInsuranceProposed,
    state::{Insurance, ReInsuranceProposal, LP},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SendInsuranceProposal<'info> {
    #[account(mut)]
    pub lp_creator: Signer<'info>,
    #[account(
        seeds = [
            lp_creator.key().as_ref()
        ],
        bump=lp.bump
    )]
    pub lp: Account<'info, LP>,
    #[account(
        seeds = [
            insurance.insurer.as_ref(),
            insurance.insurance_id.as_bytes()
        ],
        bump=insurance.bump
    )]
    pub insurance: Account<'info, Insurance>,
    #[account(
        init_if_needed,
        payer = lp_creator,
        space = 8 + ReInsuranceProposal::INIT_SPACE,
        seeds = [
            lp_creator.key().as_ref(),
            insurance.key().as_ref()
        ],
        bump
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SendInsuranceProposal>,
    proposed_commision: u64,
    proposed_undercollaterization: u64,
    proposal_docs: String,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let lp_creator = &ctx.accounts.lp_creator;
    let insurance = &ctx.accounts.insurance;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        insurance.expiry > current_time,
        InsuranceEnumError::InsuranceExpired
    );
    require!(
        !insurance.reinsured,
        InsuranceEnumError::InsuranceReinsuredAlready
    );

    proposal.bump = ctx.bumps.proposal;
    proposal.lp_owner = lp_creator.key();
    proposal.proposed_commision = proposed_commision;
    proposal.proposed_undercollaterization = proposed_undercollaterization;
    proposal.insurance = insurance.key();
    proposal.proposal_docs = proposal_docs.clone();
    proposal.proposal_accepted = false;

    emit!(ReInsuranceProposed {
        lp_owner: lp_creator.key(),
        proposed_commision: proposed_commision,
        proposed_undercollaterization: proposed_undercollaterization,
        insurance: insurance.key(),
        proposal_docs: proposal_docs
    });

    Ok(())
}
