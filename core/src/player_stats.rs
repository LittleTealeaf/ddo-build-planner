use crate::player_stats::{bonus_store::BonusStore, calc::BreakdownCache};

mod bonus_store;
mod calc;
mod inserting;

#[derive(Debug, Clone, Default)]
pub struct PlayerStats {
    bonuses: BonusStore,
    calc_cache: BreakdownCache,
}
