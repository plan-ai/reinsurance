use crate::{event::LPCreated, state::LP};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterLP<'info> {
    #[account(mut)]
    pub lp_creator: Signer<'info>,
    #[account(
        init,
        payer=lp_creator,
        space=8+LP::INIT_SPACE,
        seeds = [
            lp_creator.key().as_ref()
        ],
        bump
    )]
    pub lp: Account<'info, LP>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterLP>) -> Result<()> {
    let lp_creator = &ctx.accounts.lp_creator;
    let lp = &mut ctx.accounts.lp;

    lp.bump = ctx.bumps.lp;
    lp.lp_creator = lp_creator.key();
    lp.insures = vec![];

    emit!(LPCreated {
        lp_creator: lp_creator.key()
    });

    Ok(())
}
