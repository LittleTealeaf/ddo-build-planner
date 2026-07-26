use crate::breakdown::{bonus_store::BonusStore, calculate::BonusCache};

mod bonus_store;
mod calculate;
mod inserting;

#[derive(Debug, Clone)]
pub struct Breakdown {
    bonuses: BonusStore,
    cache: BonusCache,
}

impl Breakdown {}
