use anchor_lang::error_code;

#[error_code]
pub enum InsuranceEnumError {
    // 6000
    #[msg("Can not create insurance that exists for less than 1 month")]
    InsuranceExpiryTooClose,

    // 6001
    #[msg("Can not send reinsurance proposal on expired insurance")]
    InsuranceExpired,

    // 6002
    #[msg("Insurance already re-insured")]
    InsuranceReinsuredAlready,

    // 6003
    #[msg("Reinsurance can not be called off unless premium more than week late")]
    CanNotCallOffReinsurance,

    // 6004
    #[msg("Specified metadta outside accepted range")]
    OutsideValidRange,

    // 6005
    #[msg("LP can not fulfill under-collaterisation constraints")]
    CanNotFullFillUnderCollateralizationDemands,

    // 6006
    #[msg("Not enough voting power")]
    InsufficientVotingPower,

    // 6007
    #[msg("Voting on strategy closed!")]
    VotingOnStrategyClosed,

    // 6008
    #[msg("Not enought votes to accept")]
    NotEnoughVotes,

    // 6009
    #[msg("Can not refund before voting closes")]
    RefundDeclined,

    // 6010
    #[msg("Can not raise claim greater than coverage amount")]
    ClaimTooHigh,

    // 6011
    #[msg("Claim voting closes after 1 month")]
    ClaimVotingClosed,

    // 6012
    #[msg("Can not release decision before voting closes")]
    DecisionNotYetReleased,
}
