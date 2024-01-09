use crate::{constant::DEFAULT_MINT_DECIMALS, event::LPTokenised, state::LP};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct TokeniseLP<'info> {
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
        init_if_needed,
        payer = lp_creator,
        mint::authority = tokenised_mint,
        mint::decimals = DEFAULT_MINT_DECIMALS,
        seeds = [
            b"i_am_in_love",
            b"withacriminl",
            lp.key().as_ref()
        ],
        bump
    )]
    pub tokenised_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = lp_creator,
        associated_token::mint = tokenised_mint,
        associated_token::authority = lp_creator,
    )]
    pub lp_creator_tokenised_account: Account<'info, TokenAccount>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<TokeniseLP>,
    token_name: Option<String>,
    token_symbol: Option<String>,
    token_metadata_uri: Option<String>,
) -> Result<()> {
    let lp = &mut ctx.accounts.lp;
    let lp_creator = &mut ctx.accounts.lp_creator;
    let tokenised_mint = &mut ctx.accounts.tokenised_mint;
    let token_program = &ctx.accounts.token_program;
    let lp_creator_tokenised_account = &mut ctx.accounts.lp_creator_tokenised_account;
    let system_program = &ctx.accounts.system_program;
    let metadata = &mut ctx.accounts.token_metadata_program;
    let rent = &ctx.accounts.rent;

    let binding = lp.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"i_am_in_love",
        b"withacriminl",
        binding.as_ref(),
        &[ctx.bumps.tokenised_mint],
    ]];

    if lp.total_assets - tokenised_mint.supply > 0 {
        mint_to(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                MintTo {
                    mint: tokenised_mint.to_account_info(),
                    to: lp_creator_tokenised_account.to_account_info(),
                    authority: tokenised_mint.to_account_info(),
                },
                signer_seeds,
            ),
            lp.total_assets - tokenised_mint.supply,
        )?;
    };

    if let (Some(token_name), Some(token_symbol), Some(token_metadata_uri)) =
        (token_name, token_symbol, token_metadata_uri)
    {
        // On-chain token metadata for the mint
        let data_v2 = DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_metadata_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let cpi_ctx = CpiContext::new_with_signer(
            tokenised_mint.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: metadata.to_account_info(),
                mint: tokenised_mint.to_account_info(),
                mint_authority: tokenised_mint.to_account_info(),
                update_authority: tokenised_mint.to_account_info(),
                payer: lp_creator.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
            signer_seeds,
        );

        create_metadata_accounts_v3(cpi_ctx, data_v2, true, true, None)?;
    }

    lp.tokenised = true;
    emit!(LPTokenised { lp: lp.key() });

    Ok(())
}
