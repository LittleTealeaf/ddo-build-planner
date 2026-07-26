use im::OrdSet;

use crate::{bonus::Bonus, breakdown::Breakdown};

impl Breakdown {
    pub fn insert_bonuses<I>(&mut self, bonuses: I)
    where
        I: IntoIterator<Item = Bonus>,
    {
        let mut to_reset = OrdSet::new();
    
        while let Some(item) = to_reset.remove_min() {
            self.cache.reset_attribute(&item);
            to_reset.extend(self.bonuses.get_dependant_attributes(&item).cloned());
        }
    }
}
