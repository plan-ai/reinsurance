use anchor_lang::prelude::*;
use instructions::*;

pub mod constant;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;
pub mod strategy_program_interface;
pub mod utils;

declare_id!("8evVxMNMoHX2gdGVSLfHR4UwRUCdYC28erfpUAYaPiEn");

#[program]
pub mod insurance {
    use super::*;

    pub fn register_insurer(
        ctx: Context<RegisterInsurer>,
        verifying_documents: String,
    ) -> Result<()> {
        register_insurer::handler(ctx, verifying_documents)
    }

    pub fn register_lp(ctx: Context<RegisterLP>) -> Result<()> {
        register_lp::handler(ctx)
    }

    pub fn register_insurance(
        ctx: Context<RegisterInsurance>,
        insurance_id: String,
        coverage: u64,
        premium: u64,
        minimum_commission: u32,
        deductible: u64,
        expiry: i64,
        metadata_link: String,
    ) -> Result<()> {
        register_insurance::handler(
            ctx,
            insurance_id,
            coverage,
            premium,
            minimum_commission,
            deductible,
            expiry,
            metadata_link,
        )
    }

    pub fn accept_reinsurance_proposal(ctx: Context<AcceptReinsuranceProposal>) -> Result<()> {
        accept_reinsurance_proposal::handler(ctx)
    }

    pub fn add_security(ctx: Context<AddSecurity>, transfer_amount: u64) -> Result<()> {
        add_security::handler(ctx, transfer_amount)
    }
}
