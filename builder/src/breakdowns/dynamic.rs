use core::iter::once;
use std::collections::HashSet;

use crate::{attribute::Attribute, bonus::BonusTemplate};

use super::Breakdowns;

/// Dynamic bonuses are bonuses that need to be added after the fact, or only when a certain bonus
/// is present. This is used to allow the 'implementation' of the [`GetBonuses`] trait
/// from the data crate, where a simple implementation is not possible.
///
/// This works by storing a map between each attribute and the bonuses that should be present if
/// that attribute is greater than one. The only side effect is if that attribute is present, all
/// of it's associated bonuses are applied (leaving the conditions to dictate when it should
/// apply). This means that these bonuses will all show up in the breakdowns.
impl Breakdowns {
    /// Shortcut for a single dynamic bonus. Uses [`Self::insert_dynamic_bonuses`]
    pub fn import_dynamic_bonus<I>(&mut self, attribute: Attribute, bonuses: I)
    where
        I: IntoIterator<Item = BonusTemplate>,
    {
        self.import_dynamic_bonuses(once((attribute, bonuses)));
    }

    /// Adds dynamic bonuses to the breakdowns. This is useful for attributes that need to be
    /// included automatically, but shouldn't always be there. Bonuses are only included if the
    /// given attribute is above 0.
    pub fn import_dynamic_bonuses<I, B>(&mut self, dynamic_bonuses: I)
    where
        I: IntoIterator<Item = (Attribute, B)>,
        B: IntoIterator<Item = BonusTemplate>,
    {
        let mut attributes = HashSet::new();

        let dynamic_bonuses =
            dynamic_bonuses
                .into_iter()
                .map(|(attribute, bonuses): (Attribute, B)| {
                    attributes.insert(attribute.clone());
                    (attribute, bonuses.into_iter().collect())
                });

        self.dynamic_bonuses.extend(dynamic_bonuses);

        self.recalculate_attributes(attributes);
    }
}
