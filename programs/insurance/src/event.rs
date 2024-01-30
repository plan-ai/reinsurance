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
    pub claim_amount: u64,
}

#[event]
pub struct LPTokenised {
    pub lp: Pubkey,
}

#[event]
pub struct StrategyProposed {
    pub strategy: Pubkey,
    pub stream_amount: u64,
    pub stream_every: u64,
    pub number_of_streams: u64,
    pub premium_vault: Pubkey,
    pub strategy_id: String,
}

#[event]
pub struct StrategyVoted {
    pub strategy: Pubkey,
    pub voter: Pubkey,
    pub vote_amount: u64,
}

#[event]
pub struct StrategyAccepted {
    pub strategy: Pubkey,
}

#[event]
pub struct StrategyVoteRefunded {
    pub strategy: Pubkey,
    pub refunded_to: Pubkey,
    pub refund_amount: u64,
}

#[event]
pub struct ClaimRaised {
    pub reinsurance: Pubkey,
    pub claim: Pubkey,
    pub claim_amount: u64,
    pub claim_metadata_link: String,
}

#[event]
pub struct ClaimVoted {
    pub claim: Pubkey,
    pub voter: Pubkey,
    pub vote_amount: u64,
}

#[event]
pub struct ClaimDecisionReleased {
    pub claim: Pubkey,
    pub decision: bool,
}

#[event]
pub struct ClaimRewardReleased {
    pub claim: Pubkey,
    pub reward_amount: u64,
}

#[event]
pub struct StrategyExecuted {
    pub strategy: Pubkey,
}

#[event]
pub struct StrategyBlocked {
    pub strategy: Pubkey,
}
