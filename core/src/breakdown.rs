use crate::{
    bonuses::core_bonuses,
    breakdown::{bonus_store::BonusStore, calculate::BonusCache},
    types::bonus_provider::BonusProvider,
};

mod bonus_store;
mod calculate;
mod inserting;

#[derive(Debug, Clone, Default)]
pub struct Breakdown {
    bonuses: BonusStore,
    cache: BonusCache,
}

