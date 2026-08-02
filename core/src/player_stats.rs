use crate::player_stats::{bonuses::Bonuses, calc::BreakdownCache};

mod bonuses;
mod calc;
mod inserting;

#[derive(Debug, Clone, Default)]
pub struct PlayerStats {
    bonuses: Bonuses,
    calc_cache: BreakdownCache,
}
