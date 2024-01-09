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
}
