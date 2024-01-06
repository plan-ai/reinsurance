use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Insurer {
    pub bump: u8,
    pub insurance_creator: Pubkey,
    #[max_len(100)]
    pub verifying_documents: String,
}

#[account]
#[derive(InitSpace)]
pub struct Insurance {
    pub bump: u8,
    #[max_len(50)]
    pub insurance_id: String,
    pub insurer: Pubkey,
    pub coverage: u64,
    pub premium: u64,
    pub minimum_commission: u64,
    pub deductible: u64,
    pub expiry: i64,
    #[max_len(100)]
    pub metadata_link: String,
    pub reinsured: bool,
    pub premium_due: Option<i64>,
}

#[account]
#[derive(InitSpace)]
pub struct LP {
    pub bump: u8,
    pub lp_creator: Pubkey,
    #[max_len(20)]
    pub insures: Vec<Pubkey>,
    pub total_securitized: u64,
    pub total_tokenised: u64,
}

#[account]
#[derive(InitSpace)]
pub struct ReInsuranceProposal {
    pub bump: u8,
    pub lp_owner: Pubkey,
    pub proposed_commision: u8,
    pub proposed_undercollaterization: u8,
    pub insurance: Pubkey,
    pub proposal_accepted: bool,
}

#[account]
#[derive(InitSpace)]
pub struct PremiumVault {
    pub bump: u8,
    pub reinsurance: Pubkey,
    pub strategy_program: Option<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct SecurtyVault {
    pub bump: u8,
    pub reinsurance: Pubkey,
}
