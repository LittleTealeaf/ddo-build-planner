use im::OrdSet;

use crate::{bonus::Bonus, breakdown::Breakdown, types::bonus_provider::BonusProvider};

impl Breakdown {
    pub fn insert_bonuses<I>(&mut self, bonuses: I, provider: BonusProvider)
    where
        I: IntoIterator<Item = Bonus>,
    {
        let mut to_reset = OrdSet::new();

        self.bonuses.insert_bonuses(bonuses, provider, snapshot);
    
        while let Some(item) = to_reset.remove_min() {
            self.cache.reset_attribute(&item);
            to_reset.extend(self.bonuses.get_dependant_attributes(&item).cloned());
        }
    }
}
