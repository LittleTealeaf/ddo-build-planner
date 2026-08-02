use core::iter::{empty, once};

use crate::{bonus::Bonus, player_stats::PlayerStats, types::bonus_provider::BonusProvider};

impl PlayerStats {
    pub fn clear_provider(&mut self, provider: &BonusProvider) {
        self.insert_bonuses(empty(), provider, None);
    }

    pub fn insert_bonus(&mut self, bonus: Bonus, provider: &BonusProvider, snapshot: Option<u32>) {
        self.insert_bonuses(once(bonus), provider, snapshot);
    }

    pub fn insert_bonuses<I>(&mut self, bonuses: I, provider: &BonusProvider, snapshot: Option<u32>)
    where
        I: IntoIterator<Item = Bonus>,
    {
        let attributes = self
            .bonuses
            .insert_bonuses(bonuses, provider, snapshot)
            .collect::<Vec<_>>();
        self.calculator().reset_attributes(attributes);
    }
}
