use crate::{
    constant::TWO_WEEKS,
    event::PremiumPayed,
    state::{Insurance, PremiumVault, ReInsuranceProposal},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(premium_multiplier:u64)]
pub struct PayPremiun<'info> {
    #[account(mut)]
    pub insurance_creator: Signer<'info>,
    #[account(
        mut,
        constraint = insurance_creator_token_account.mint==usdc_mint.key(),
        constraint = insurance_creator_token_account.owner == insurance_creator.key(),
        constraint = insurance_creator_token_account.amount >= premium_multiplier * insurance.premium
    )]
    pub insurance_creator_token_account: Account<'info, TokenAccount>,
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
        init_if_needed,
        payer = insurance_creator,
        space = 8 + PremiumVault::INIT_SPACE,
        seeds = [
            b"premium",
            insurance.key().as_ref(),
            proposal.key().as_ref()
        ],
        bump
    )]
    pub premium_vault: Account<'info, PremiumVault>,
    #[account(
        init_if_needed,
        payer = insurance_creator,
        associated_token::mint = usdc_mint,
        associated_token::authority = premium_vault
    )]
    pub premium_vault_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            proposal.lp_owner.key().as_ref(),
            insurance.key().as_ref()
        ],
        bump=proposal.bump,
        constraint = proposal.proposal_accepted == true
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayPremiun>, premium_multiplier: u64) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let premium_vault = &mut ctx.accounts.premium_vault;
    let insurance = &mut ctx.accounts.insurance;
    let proposal = &ctx.accounts.proposal;
    let insurance_creator = &mut ctx.accounts.insurance_creator;
    let insurance_creator_token_account = &mut ctx.accounts.insurance_creator_token_account;

    premium_vault.bump = ctx.bumps.premium_vault;
    premium_vault.reinsurance = proposal.key();

    let transfer_amount = premium_multiplier
        * (insurance.premium * ((1 - (proposal.proposed_commision / 100)) as u64));

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: insurance_creator_token_account.to_account_info(),
                to: premium_vault.to_account_info(),
                authority: insurance_creator.to_account_info(),
            },
        ),
        transfer_amount,
    )?;

    insurance.premium_due =
        Some(insurance.premium_due.unwrap() + (premium_multiplier as i64) * TWO_WEEKS);

    emit!(PremiumPayed {
        reinsurance: proposal.key(),
        prepayment_time: insurance.premium_due.unwrap()
    });

    Ok(())
}
