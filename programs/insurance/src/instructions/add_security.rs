use crate::{event::LPAssetAdded, state::LP};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(transfer_amount:u64)]
pub struct AddSecurity<'info> {
    #[account(mut)]
    pub lp_creator: Signer<'info>,
    #[account(
        mut,
        seeds = [
            lp_creator.key().as_ref()
        ],
        bump=lp.bump
    )]
    pub lp: Account<'info, LP>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = lp_creator,
        constraint = lp_creator_usdc_account.amount >= transfer_amount
    )]
    pub lp_creator_usdc_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = lp_creator,
        associated_token::mint = usdc_mint,
        associated_token::authority = lp
    )]
    pub lp_usdc_account: Account<'info, TokenAccount>,
    // #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddSecurity>, transfer_amount: u64) -> Result<()> {
    let lp = &mut ctx.accounts.lp;
    let lp_creator = &mut ctx.accounts.lp_creator;
    let lp_creator_usdc_account = &mut ctx.accounts.lp_creator_usdc_account;
    let lp_usdc_account = &mut ctx.accounts.lp_usdc_account;
    let token_program = &ctx.accounts.token_program;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: lp_creator_usdc_account.to_account_info(),
                to: lp_usdc_account.to_account_info(),
                authority: lp_creator.to_account_info(),
            },
        ),
        transfer_amount,
    )?;

    lp.total_assets += transfer_amount;

    emit!(LPAssetAdded {
        lp: lp.key(),
        asset_amount: transfer_amount
    });

    Ok(())
}
