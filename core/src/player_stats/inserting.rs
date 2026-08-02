use crate::{bonus::Bonus, player_stats::PlayerStats, types::bonus_provider::BonusProvider};

impl PlayerStats {
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
