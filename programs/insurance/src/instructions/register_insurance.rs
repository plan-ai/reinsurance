use crate::{
    constant::MONTH,
    error::InsuranceEnumError,
    event::InsuranceCreated,
    state::{Insurance, Insurer},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(insurance_id:String)]
pub struct RegisterInsurance<'info> {
    #[account(mut)]
    pub insurance_creator: Signer<'info>,
    #[account(
        seeds = [
            insurance_creator.key().as_ref()
        ],
        bump=insurer.bump
    )]
    pub insurer: Account<'info, Insurer>,
    #[account(
        init,
        payer = insurance_creator,
        space = 8 + Insurance::INIT_SPACE,
        seeds = [
            insurance_creator.key().as_ref(),
            insurance_id.as_bytes()
        ],
        bump
    )]
    pub insurance: Account<'info, Insurance>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterInsurance>,
    insurance_id: String,
    coverage: u64,
    premium: u64,
    minimum_commission: u64,
    deductible: u64,
    expiry: i64,
    metadata_link: String,
) -> Result<()> {
    let insurance = &mut ctx.accounts.insurance;
    let insurance_creator = &ctx.accounts.insurance_creator;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        expiry >= current_time + MONTH,
        InsuranceEnumError::InsuranceExpiryTooClose
    );
    insurance.bump = ctx.bumps.insurance;
    insurance.insurance_id = insurance_id.clone();
    insurance.insurer = insurance_creator.key();
    insurance.coverage = coverage;
    insurance.premium = premium;
    insurance.minimum_commission = minimum_commission;
    insurance.deductible = deductible;
    insurance.expiry = expiry;
    insurance.metadata_link = metadata_link.clone();
    insurance.reinsured = false;
    insurance.premium_due = None;

    emit!(InsuranceCreated {
        insurer: insurance_creator.key(),
        insurance_id: insurance_id,
        coverage: coverage,
        premium: premium,
        minimum_commission: minimum_commission,
        deductible: deductible,
        expiry: expiry,
        metadata_link: metadata_link,
    });

    Ok(())
}
