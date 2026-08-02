use crate::{bonus::Bonus, breakdown::Breakdown, types::bonus_provider::BonusProvider};

impl Breakdown {
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
