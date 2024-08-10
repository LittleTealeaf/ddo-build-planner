use std::collections::HashMap;

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
    /// List of stacking bonuses that are always applied to the attribute
    #[must_use]
    pub const fn stacking(&self) -> &Vec<BonusEntry> {
        &self.stacking
    }

    /// List of stacking bonuses that are always applied to the attribute, but are currently
    /// disabled, meaning that their conditions return false
    #[must_use]
    pub const fn disabled_stacking(&self) -> &Vec<BonusEntry> {
        &self.disabled_stacking
    }

    /// List of bonus entries. Each item indicates a particular bonus type
    #[must_use]
    pub const fn bonuses(&self) -> &Vec<BonusTypeEntry> {
        &self.bonuses
    }

    /// The final reuslitng value of the attribute
    #[must_use]
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
    /// The bonus type tracked in this entry
    #[must_use]
    pub const fn bonus_type(&self) -> BonusType {
        self.bonus_type
    }

    /// The final applied bonus for this bonus type. This is the highest value of this bonus type
    #[must_use]
    pub const fn applied(&self) -> &Option<BonusEntry> {
        &self.applied
    }

    /// Bonsues that do not provide the highest value
    #[must_use]
    pub const fn overwritten(&self) -> &Vec<BonusEntry> {
        &self.overwritten
    }

    /// Bonuses whose condition returns false
    #[must_use]
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
    /// The bonus object itself
    #[must_use]
    pub const fn bonus(&self) -> &Bonus {
        &self.bonus
    }

    /// What the condition evaluates to
    #[must_use]
    pub const fn condition(&self) -> bool {
        self.condition
    }

    /// What the value evaluates to
    #[must_use]
    pub const fn value(&self) -> &Decimal {
        &self.value
    }
}

impl Breakdowns {
    /// Adds a specific attribute to be tracked. Attributes that are tracked will always have a
    /// cached version of their breakdown (accessible by immutable references) via
    /// [`get_breakdowns()`]. This cached version is updated whenever a change is made.
    ///
    /// [`get_breakdowns()`]: crate::breakdowns::Breakdowns::get_breakdowns
    pub fn add_breakdown<A>(&mut self, attribute: A)
    where
        A: Into<Attribute>,
    {
        let attribute = Attribute::from_into(attribute);
        let breakdown = self.build_breakdown(&attribute);
        self.cache.breakdowns.insert(attribute, breakdown);
    }

    /// Returns a reference to the map of tracked breakdowns
    #[must_use]
    pub const fn breakdowns(&self) -> &HashMap<Attribute, AttributeBreakdown> {
        &self.cache.breakdowns
    }

    /// Removes and clears all attributes from being cached in breakdowns
    pub fn clear_breakdowns(&mut self) {
        self.cache.breakdowns.clear();
    }

    /// Removes an attribute's breakdown from being cached
    pub fn remove_breakdown(&mut self, attribute: &Attribute) -> Option<AttributeBreakdown> {
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
            .partition(BonusEntry::condition);

        let bonuses = attribute_bonuses
            .into_iter()
            .map(|(bonus_type, bonuses)| {
                let (mut bonuses, disabled): (Vec<_>, Vec<_>) =
                    bonuses.into_iter().partition(BonusEntry::condition);

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
