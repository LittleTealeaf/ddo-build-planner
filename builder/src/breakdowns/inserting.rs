use crate::bonus::{Bonus, BonusSource};

use super::Breakdowns;

impl Breakdowns {
    /// Removes all bonuses with any of the provided [`BonusSources`]
    ///
    /// [`BonusSources`]: BonusSource
    pub fn remove_sources(&mut self, sources: impl IntoIterator<Item = BonusSource>) {
        self.insert_bonuses(sources.into_iter().map(Bonus::dummy));
    }

    /// Removes all bonuses with the provided [`BonusSource`]
    pub fn remove_source(&mut self, source: BonusSource) {
        self.remove_sources([source]);
    }

    /// Inserts a single bonus into the breakdowns. This also removes all bonuses that have the
    /// same bonus source.
    pub fn insert_bonus(&mut self, bonus: Bonus) {
        self.insert_bonuses([bonus]);
    }

    /// Inserts several bonuses into the breakdowns. This also removes all bonuses that have the
    /// same bonus source.
    pub fn insert_bonuses(&mut self, bonuses: impl IntoIterator<Item = Bonus>) {
        todo!()
    }
}
