use crate::breakdown::{bonus_store::BonusStore, calc::BreakdownCache};

mod bonus_store;
mod calc;
mod inserting;

#[derive(Debug, Clone, Default)]
pub struct Breakdown {
    bonuses: BonusStore,
    calc_cache: BreakdownCache,
}
