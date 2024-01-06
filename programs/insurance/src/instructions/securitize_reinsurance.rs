use crate::{
    error::InsuranceEnumError,
    event::ReInsuranceSecuritized,
    state::{Insurance, ReInsuranceProposal, SecurtyVault, LP},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct SecuritizeReInsurance<'info> {
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
        constraint=lp_usdc_account.mint==usdc_mint.key(),
        constraint = lp_usdc_account.owner == lp_creator.key()
    )]
    pub lp_usdc_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            proposal.lp_owner.as_ref(),
            proposal.insurance.as_ref()
        ],
        bump=proposal.bump,
        constraint = proposal.proposal_accepted == true
    )]
    pub proposal: Account<'info, ReInsuranceProposal>,
    #[account(
        mut,
        constraint = insurance.key() == proposal.insurance
    )]
    pub insurance: Account<'info, Insurance>,
    #[account(
        init,
        payer = lp_creator,
        space = 8 + SecurtyVault::INIT_SPACE,
        seeds = [
            b"security",
            insurance.key().as_ref(),
            proposal.key().as_ref()
        ],
        bump
    )]
    pub security_vault: Account<'info, SecurtyVault>,
    #[account(
        init,
        payer = lp_creator,
        associated_token::mint = usdc_mint,
        associated_token::authority = security_vault
    )]
    pub security_vault_token_account: Account<'info, TokenAccount>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SecuritizeReInsurance>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let lp_usdc_account = &mut ctx.accounts.lp_usdc_account;
    let security_vault_token_account = &mut ctx.accounts.security_vault_token_account;
    let lp = &mut ctx.accounts.lp;
    let proposal = &ctx.accounts.proposal;
    let insurance = &mut ctx.accounts.insurance;
    let security_vault = &mut ctx.accounts.security_vault;

    require!(
        !insurance.reinsured,
        InsuranceEnumError::InsuranceReinsuredAlready
    );

    let transfer_amount =
        ((proposal.proposed_undercollaterization as u64) * insurance.coverage) / 100;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: lp_usdc_account.to_account_info(),
                to: security_vault_token_account.to_account_info(),
                authority: lp.to_account_info(),
            },
        ),
        transfer_amount,
    )?;

    security_vault.bump = ctx.bumps.security_vault;
    security_vault.reinsurance = proposal.key();

    let current_time = Clock::get()?.unix_timestamp;
    insurance.reinsured = true;
    insurance.premium_due = Some(current_time);
    lp.insures.push(insurance.key());
    lp.total_securitized += transfer_amount;

    emit!(ReInsuranceSecuritized {
        reinsurance: proposal.key(),
        security_vault: security_vault.key()
    });

    Ok(())
}
