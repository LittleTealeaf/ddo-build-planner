use crate::breakdown::{bonus_store::BonusStore, calculate::CalcCache, details::DetailsCache};

mod bonus_store;
mod calculate;
mod details;
mod inserting;

#[derive(Debug, Clone, Default)]
pub struct Breakdown {
    bonuses: BonusStore,
    calc_cache: CalcCache,
    details: DetailsCache,
}
