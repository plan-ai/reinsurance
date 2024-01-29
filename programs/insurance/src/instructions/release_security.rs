use crate::{
    event::ReInsuranceClaimed,
    state::{Claim, Insurance, ReInsuranceProposal, LP},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct ReleaseSecurity<'info> {
    pub lp_creator: Signer<'info>,
    /// CHECK: Seed checks done later
    pub insurance_creator: AccountInfo<'info>,
    #[account(
        mut,
        constraint = insurance_creator_token_account.mint==usdc_mint.key(),
        constraint = insurance_creator_token_account.owner == insurance_creator.key(),
    )]
    pub insurance_creator_token_account: Account<'info, TokenAccount>,
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
        associated_token::authority = lp
    )]
    pub lp_usdc_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            insurance_creator.key().as_ref(),
            insurance.insurance_id.as_bytes()
        ],
        bump=insurance.bump,
        constraint = insurance.reinsured == true
    )]
    pub insurance: Account<'info, Insurance>,
    #[account(
        seeds = [
            lp_creator.key().as_ref(),
            insurance.key().as_ref()
        ],
        bump=proposal.bump,
        constraint = proposal.proposal_accepted == true
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"claim",
            proposal.key().as_ref(),
            claim.claim_id.as_bytes()
        ],
        bump=claim.bump,
        constraint = claim.accepted.unwrap() == true,
        constraint = claim.claimed == false
    )]
    pub claim: Account<'info, Claim>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ReleaseSecurity>) -> Result<()> {
    let lp = &mut ctx.accounts.lp;
    let lp_usdc_account = &mut ctx.accounts.lp_usdc_account;
    let lp_creator = &ctx.accounts.lp_creator;
    let insurance_creator_token_account = &mut ctx.accounts.insurance_creator_token_account;
    let proposal = &ctx.accounts.proposal;
    let claim = &mut ctx.accounts.claim;
    let token_program = &ctx.accounts.token_program;

    let transfer_amount = claim.claim_amount;

    let binding = lp_creator.key();
    let lp_signer_seeds: &[&[&[u8]]] = &[&[binding.as_ref(), &[lp.bump]]];

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: lp_usdc_account.to_account_info(),
                to: insurance_creator_token_account.to_account_info(),
                authority: lp.to_account_info(),
            },
            lp_signer_seeds,
        ),
        transfer_amount,
    )?;

    lp.total_assets -= transfer_amount;
    claim.claimed = true;

    emit!(ReInsuranceClaimed {
        reinsurance: proposal.key(),
        claim_amount: transfer_amount
    });

    Ok(())
}
