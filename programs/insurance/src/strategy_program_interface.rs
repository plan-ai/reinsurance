use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

static IDS: [Pubkey; 1] = [pubkey!("55kBY9yxqSC42boV8PywT2gqGzgLi5MPAtifNRgPNezF")];

#[derive(Clone)]
pub struct StrategyInterface;

impl anchor_lang::Ids for StrategyInterface {
    fn ids() -> &'static [Pubkey] {
        &IDS
    }
}
