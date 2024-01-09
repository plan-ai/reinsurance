use anchor_lang::prelude::*;

#[event]
pub struct InsurerRegistered {
    pub insurance_creator: Pubkey,
    pub verifying_documents: String,
}

#[event]
pub struct InsuranceCreated {
    pub insurer: Pubkey,
    pub insurance_id: String,
    pub coverage: u64,
    pub premium: u64,
    pub minimum_commission: u32,
    pub deductible: u64,
    pub expiry: i64,
    pub metadata_link: String,
}

#[event]
pub struct LPCreated {
    pub lp_creator: Pubkey,
}

#[event]
pub struct LPAssetAdded {
    pub lp: Pubkey,
    pub asset_amount: u64,
}

#[event]
pub struct ReInsuranceProposed {
    pub lp_owner: Pubkey,
    pub proposed_commision: u64,
    pub proposed_undercollaterization: u64,
    pub insurance: Pubkey,
    pub proposal_docs: String,
}

#[event]
pub struct ReInsuranceProposalAccepted {
    pub reinsurance: Pubkey,
}

#[event]
pub struct ReInsuranceCalledOff {
    pub reinsurance: Pubkey,
}

#[event]
pub struct PremiumPayed {
    pub reinsurance: Pubkey,
    pub prepayment_time: i64,
}

#[event]
pub struct ReInsuranceClaimed {
    pub reinsurance: Pubkey,
}

#[event]
pub struct LPTokenised {
    pub lp: Pubkey,
}

#[event]
pub struct StrtegySelected {
    pub reinsurance: Pubkey,
    pub strategy: Pubkey,
}
