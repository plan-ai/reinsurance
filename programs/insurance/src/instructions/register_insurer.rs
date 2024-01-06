use crate::{event::InsurerRegistered, state::Insurer};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterInsurer<'info> {
    #[account(mut)]
    pub insurance_creator: Signer<'info>,
    #[account(
        init,
        payer=insurance_creator,
        space = 8+Insurer::INIT_SPACE,
        seeds = [
            insurance_creator.key().as_ref()
        ],
        bump
    )]
    pub insurer: Account<'info, Insurer>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterInsurer>, verifying_documents: String) -> Result<()> {
    let insurer = &mut ctx.accounts.insurer;
    let insurance_creator = &ctx.accounts.insurance_creator;

    insurer.bump = ctx.bumps.insurer;
    insurer.insurance_creator = insurance_creator.key();
    insurer.verifying_documents = verifying_documents.clone();

    emit!(InsurerRegistered {
        insurance_creator: insurance_creator.key(),
        verifying_documents: verifying_documents,
    });

    Ok(())
}
