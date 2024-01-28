use crate::{
    constant::TWO_WEEKS,
    error::InsuranceEnumError,
    event::ReInsuranceCalledOff,
    state::{Insurance, PremiumVault, ReInsuranceProposal, LP},
    utils::remove_from_vector,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct CallOffReinsurance<'info> {
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
        mut,
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
        constraint = insurance.key() == proposal.insurance,
        constraint = insurance.reinsured == true
    )]
    pub insurance: Account<'info, Insurance>,
    #[account(
        mut,
        seeds = [
            b"premium",
            insurance.key().as_ref(),
            proposal.key().as_ref()
        ],
        bump=premium_vault.bump
    )]
    pub premium_vault: Option<Account<'info, PremiumVault>>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = premium_vault
    )]
    pub premium_vault_token_account: Option<Account<'info, TokenAccount>>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CallOffReinsurance>) -> Result<()> {
    let lp = &mut ctx.accounts.lp;
    let insurance = &mut ctx.accounts.insurance;
    let proposal = &mut ctx.accounts.proposal;
    let current_time = Clock::get()?.unix_timestamp;
    let token_program = &ctx.accounts.token_program;
    let lp_usdc_account = &mut ctx.accounts.lp_usdc_account;
    let premium_vault = &mut ctx.accounts.premium_vault;
    let premium_vault_token_account = &mut ctx.accounts.premium_vault_token_account;

    require!(
        current_time > insurance.premium_due.unwrap() + TWO_WEEKS
            || insurance.expiry <= current_time,
        InsuranceEnumError::CanNotCallOffReinsurance
    );

    insurance.reinsured = false;
    proposal.proposal_accepted = false;
    lp.total_securitized -= insurance.coverage;

    remove_from_vector(insurance.key(), &mut lp.insures).unwrap();
    remove_from_vector(
        proposal.proposed_undercollaterization,
        &mut lp.undercollaterization_promised,
    )
    .unwrap();

    if let (Some(premium_vault), Some(premium_vault_token_account)) =
        (premium_vault, premium_vault_token_account)
    {
        let proposal_binding = proposal.key();
        let insurance_binding = insurance.key();
        let premium_vault_signer_seeds: &[&[&[u8]]] = &[&[
            b"premium",
            insurance_binding.as_ref(),
            proposal_binding.as_ref(),
            &[premium_vault.bump],
        ]];

        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: premium_vault_token_account.to_account_info(),
                    to: lp_usdc_account.to_account_info(),
                    authority: premium_vault.to_account_info(),
                },
                premium_vault_signer_seeds,
            ),
            premium_vault_token_account.amount,
        )?;
    };

    emit!(ReInsuranceCalledOff {
        reinsurance: proposal.key(),
    });

    Ok(())
}
