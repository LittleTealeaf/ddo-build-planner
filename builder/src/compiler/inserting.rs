use crate::bonus::{Bonus, BonusSource, CloneBonus};

use super::{attribute_queue::AttributeQueue, Compiler};

/// Proxy Functions for Adding Bonuses
impl Compiler {
    pub fn remove_source(&mut self, source: BonusSource) {
        self.add_bonuses(vec![Bonus::dummy(source)])
    }

    pub fn add_bonus(&mut self, bonus: Bonus) {
        self.add_bonuses(vec![bonus])
    }
}

/// Adding bonsues
impl Compiler {
    pub fn add_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
        expand_cloned_bonuses(&mut bonuses);

        let mut attribute_queue =
            AttributeQueue::initialize(bonuses.iter().map(Bonus::get_attribute).collect(), false);

    }
}

/// Expands a vec of bonuses to also include any cloned bonuses
fn expand_cloned_bonuses(bonuses: &mut Vec<Bonus>) {
    bonuses.append(
        &mut bonuses
            .iter()
            .filter_map(|bonus| bonus.get_attribute().clone_bonus(bonus))
            .flatten()
            .collect(),
    )
}
