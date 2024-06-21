use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::{from_into::FromInto, hashmap::IntoGroupedHashMap};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

use super::Breakdowns;

/// Provides the value breakdown of a particular attribute
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeBreakdown {
    stacking: Vec<BonusEntry>,
    disabled_stacking: Vec<BonusEntry>,
    bonuses: Vec<BonusTypeEntry>,
    value: Decimal,
}

impl AttributeBreakdown {
    pub const fn stacking(&self) -> &Vec<BonusEntry> {
        &self.stacking
    }

    pub const fn disabled_stacking(&self) -> &Vec<BonusEntry> {
        &self.disabled_stacking
    }

    pub const fn bonuses(&self) -> &Vec<BonusTypeEntry> {
        &self.bonuses
    }

    pub const fn value(&self) -> &Decimal {
        &self.value
    }
}

/// Provides the details regarding the breakdown of a particular bonus type
#[derive(PartialEq, Eq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct BonusTypeEntry {
    bonus_type: BonusType,
    applied: Option<BonusEntry>,
    overwritten: Vec<BonusEntry>,
    disabled: Vec<BonusEntry>,
}

impl BonusTypeEntry {
    pub const fn bonus_type(&self) -> BonusType {
        self.bonus_type
    }

    pub const fn applied(&self) -> &Option<BonusEntry> {
        &self.applied
    }

    pub const fn overwritten(&self) -> &Vec<BonusEntry> {
        &self.overwritten
    }

    pub const fn disabled(&self) -> &Vec<BonusEntry> {
        &self.disabled
    }
}

/// Provides the details regarding the breakdown of a particular bonus
#[derive(PartialEq, Eq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct BonusEntry {
    bonus: Bonus,
    condition: bool,
    value: Decimal,
}

impl BonusEntry {
    pub const fn bonus(&self) -> &Bonus {
        &self.bonus
    }

    pub const fn condition(&self) -> bool {
        self.condition
    }

    pub const fn value(&self) -> &Decimal {
        &self.value
    }
}

impl Breakdowns {
    /// Adds a specific attribute to be tracked. Attributes that are tracked will always have a
    /// cached version of their breakdown (accessible by immutable references) via
    /// [`get_breakdown()`]. This cached version is updated whenever a change is made.
    ///
    /// [`get_breakdown()`]: crate::breakdowns::Breakdowns::get_breakdown
    pub fn track_breakdown<A>(&mut self, attribute: A)
    where
        A: Into<Attribute>,
    {
        let attribute = Attribute::from_into(attribute);
        let breakdown = self.build_breakdown(&attribute);
        self.cache.breakdowns.insert(attribute, breakdown);
    }

    /// Returns a reference to the breakdown if it exists for the given attribute.
    #[must_use]
    pub fn get_breakdown(&self, attribute: &Attribute) -> Option<&AttributeBreakdown> {
        self.cache.breakdowns.get(attribute)
    }

    /// Returns whether or not the given attribute is currently tracked
    #[must_use]
    pub fn is_tracked(&self, attribute: &Attribute) -> bool {
        self.cache.breakdowns.contains_key(attribute)
    }

    /// Removes and clears all attributes from being cached in breakdowns
    pub fn clear_breakdown(&mut self) {
        self.cache.breakdowns.clear();
    }

    /// Removes an attribute's breakdown from being cached
    pub fn untrack_breakdown(&mut self, attribute: &Attribute) -> Option<AttributeBreakdown> {
        self.cache.breakdowns.remove(attribute)
    }

    pub(super) fn build_breakdown(&mut self, attribute: &Attribute) -> AttributeBreakdown {
        let value = self.evaluate_attribute(attribute);

        let mut attribute_bonuses = self
            .bonuses
            .get(attribute)
            .cloned()
            .into_iter()
            .flatten()
            .map(|bonus| {
                (
                    *bonus.bonus_type(),
                    BonusEntry {
                        value: self.evaluate_value(bonus.value()),
                        condition: self.evaluate_condition(bonus.condition()),
                        bonus,
                    },
                )
            })
            .into_grouped_hash_map();

        let (stacking, disabled_stacking) = attribute_bonuses
            .remove(&BonusType::Stacking)
            .unwrap_or_default()
            .into_iter()
            .partition(|bonus| bonus.condition);

        let bonuses = attribute_bonuses
            .into_iter()
            .map(|(bonus_type, bonuses)| {
                let (mut bonuses, disabled): (Vec<_>, Vec<_>) =
                    bonuses.into_iter().partition(|bonus| bonus.condition);

                let max = bonuses
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, bonus)| bonus.value)
                    .map(|(index, _)| index);

                let applied = max.map(|index| bonuses.swap_remove(index));

                BonusTypeEntry {
                    overwritten: bonuses,
                    bonus_type,
                    applied,
                    disabled,
                }
            })
            .collect();

        AttributeBreakdown {
            stacking,
            disabled_stacking,
            bonuses,
            value,
        }
    }
}
