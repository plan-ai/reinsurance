use crate::{
    error::InsuranceEnumError,
    event::StrategyExecuted,
    state::{Insurance, PremiumVault, ReInsuranceProposal, StrategyAccount},
    strategy_program_interface::StrategyInterface,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{Mint, Token, TokenAccount},
};
use strategy::cpi::accounts::ExecuteStrategyCPI;

#[derive(Accounts)]
#[instruction(stream_amount: u64)]
pub struct ExecuteStrategy<'info> {
    pub executor: Signer<'info>,
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
    pub premium_vault: Account<'info, PremiumVault>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = premium_vault,
        constraint = premium_vault_token_account.amount >= stream_amount
    )]
    pub premium_vault_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            b"strategy",
            proposed_strategy.strategy_id.as_bytes(),
            premium_vault.key().as_ref()
        ],
        bump=proposed_strategy.bump,
        constraint = proposed_strategy.strategy_accepted == true
    )]
    pub proposed_strategy: Account<'info, StrategyAccount>,
    #[account(
        constraint = strategy_program.key() == proposed_strategy.strategy_program
    )]
    pub strategy_program: Interface<'info, StrategyInterface>,
    ///CHECK: account on which strategy money is deposited
    pub executor_account: AccountInfo<'info>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ExecuteStrategy>) -> Result<()> {
    let strategy_program = ctx.accounts.strategy_program.to_account_info();
    let premium_vault = &mut ctx.accounts.premium_vault;
    let premium_vault_token_account = &mut ctx.accounts.premium_vault_token_account;
    let usdc_mint = &ctx.accounts.usdc_mint;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let executor_account = &ctx.accounts.executor_account;
    let proposal = &ctx.accounts.proposal;
    let proposed_strategy = &mut ctx.accounts.proposed_strategy;
    let insurance = &ctx.accounts.insurance;
    let system_program = &ctx.accounts.system_program;

    let current_time = Clock::get()?.unix_timestamp;
    match proposed_strategy.last_stream_payment {
        None => {
            proposed_strategy.last_stream_payment = Some(current_time);
        }
        Some(last_stream_payment) => {
            require!(
                current_time - last_stream_payment >= proposed_strategy.stream_every,
                InsuranceEnumError::StreamMaturationNotYetReached
            );
            proposed_strategy.last_stream_payment =
                Some(last_stream_payment + proposed_strategy.stream_every);
        }
    };

    require!(
        proposed_strategy.number_of_streams > 0,
        InsuranceEnumError::StrategyStreamsEnded
    );
    proposed_strategy.number_of_streams -= 1;

    require!(
        proposed_strategy.strategy_blocked == false,
        InsuranceEnumError::StrategyBlocked
    );

    let intital_balance = premium_vault_token_account.amount;
    let strategy_accounts = ExecuteStrategyCPI {
        strategy_executor: premium_vault.to_account_info(),
        premium_vault: premium_vault.to_account_info(),
        premium_vault_token_account: premium_vault_token_account.to_account_info(),
        executor_account: executor_account.to_account_info(),
        usdc_mint: usdc_mint.to_account_info(),
        token_program: token_program.to_account_info(),
        associated_token_program: associated_token_program.to_account_info(),
        system_program: system_program.to_account_info(),
    };

    let proposal_binding = proposal.key();
    let insurance_binding = insurance.key();
    let premium_vault_signer_seeds: &[&[&[u8]]] = &[&[
        b"premium",
        insurance_binding.as_ref(),
        proposal_binding.as_ref(),
        &[premium_vault.bump],
    ]];

    let cpi_ctx = CpiContext::new_with_signer(
        strategy_program,
        strategy_accounts,
        premium_vault_signer_seeds,
    );
    strategy::cpi::execute_strategy(cpi_ctx, proposed_strategy.stream_amount)?;
    premium_vault_token_account.reload()?;
    let final_balance = premium_vault_token_account.amount;
    require!(
        intital_balance - final_balance <= proposed_strategy.stream_amount,
        InsuranceEnumError::StrategyAllocationTooHigh
    );

    emit!(StrategyExecuted {
        strategy: proposed_strategy.key()
    });

    Ok(())
}
