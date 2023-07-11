use crate::bonus::{Bonus, BonusSource, CloneBonus};

use super::Compiler;

/// Proxy Functions for Adding Bonuses
impl Compiler {
    /// Removes all bonuses from a given source from the compiler
    pub fn remove_source(&mut self, source: BonusSource) {
        self.add_bonuses(vec![Bonus::dummy(source)]);
    }

    /// Adds a single bonus to the compiler
    pub fn add_bonus(&mut self, bonus: Bonus) {
        self.add_bonuses(vec![bonus]);
    }
}

/// Adding bonsues
impl Compiler {
    /// Adds multiple bonuses to the compiler
    pub fn add_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
        expand_cloned_bonuses(&mut bonuses);
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
    );
}
