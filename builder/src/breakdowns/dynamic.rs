use core::iter::{empty, once, Empty};
use std::collections::HashSet;

use crate::{
    attribute::Attribute,
    bonus::{BonusTemplate, ToValue},
};

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
    pub fn import_dynamic_bonus<D>(&mut self, dynamic_bonus: D)
    where
        D: DynamicBonus,
    {
        self.import_dynamic_bonuses(once(dynamic_bonus));
    }

    /// Adds dynamic bonuses to the breakdowns. This is useful for attributes that need to be
    /// included automatically, but shouldn't always be there. Bonuses are only included if the
    /// given attribute is above 0.
    pub fn import_dynamic_bonuses<D, I>(&mut self, dynamic_bonuses: I)
    where
        D: DynamicBonus,
        I: IntoIterator<Item = D>,
    {
        let mut attributes = HashSet::new();

        let dynamic_bonuses = dynamic_bonuses.into_iter().map(|dynamic_bonus| {
            let attribute = dynamic_bonus.attribute();
            attributes.insert(attribute.clone());

            let custom = dynamic_bonus.custom_bonuses();
            let tiered = dynamic_bonus
                .tiered_bonuses()
                .flat_map(move |(tier, bonuses)| {
                    let condition = attribute
                        .clone()
                        .to_value()
                        .greater_or_equal_to(tier.to_value());
                    bonuses
                        .into_iter()
                        .map(move |bonus| bonus.with_condition_and(condition.clone()))
                });

            let bonuses = custom.chain(tiered);

            (dynamic_bonus.attribute(), bonuses.collect())
        });

        self.dynamic_bonuses.extend(dynamic_bonuses);
        self.recalculate_attributes(attributes);
    }
}

/// Trait implementation to allow importing as a dynamic bonus with [`Breakdowns::import_dynamic_bonus`] or [`Breakdowns::import_dynamic_bonuses`]
///
/// Either implement [`DynamicBonus::custom_bonuses`] or [`DynamicBonus::tiered_bonuses`] to fully implement.
pub trait DynamicBonus {
    /// The attribute that the dynamic bonuses are associated with
    fn attribute(&self) -> Attribute;
    /// Any custom bonuses that don't fall under the [`DynamicBonus::tiered_bonuses`] format. Returns some iterator of bonus templates
    fn custom_bonuses(&self) -> impl Iterator<Item = BonusTemplate> {
        empty()
    }

    /// Returns an iterator of the minimum attribute value, and it's corresponding list of bonuses that should be applied
    fn tiered_bonuses(
        &self,
    ) -> impl Iterator<Item = (i32, impl IntoIterator<Item = BonusTemplate>)> {
        empty::<(i32, Empty<BonusTemplate>)>()
    }
}
