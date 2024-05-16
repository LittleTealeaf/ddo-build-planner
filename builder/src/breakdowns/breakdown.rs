use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

use super::Breakdowns;

/// Provides both the [`Bonus`] as well as the calculated [`Value`].
///
/// [`Value`]: crate::bonus::Value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BonusEntry {
    bonus: Bonus,
    value: Decimal,
}

impl BonusEntry {
    /// Returns the [`Bonus`] object
    #[must_use]
    pub const fn bonus(&self) -> &Bonus {
        &self.bonus
    }

    /// Returns the calculated [`Value`] as a [`Decimal`]
    ///
    /// [`Value`]: crate::bonus::Value
    #[must_use]
    pub const fn value(&self) -> &Decimal {
        &self.value
    }
}

/// Provides a complete breakdown of an attribute
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttributeBreakdown {
    applied: Vec<BonusEntry>,
    overwritten: Vec<BonusEntry>,
    disabled: Vec<BonusEntry>,
    value: Decimal,
}

impl AttributeBreakdown {
    /// Returns the active and applied [`BonusEntry`] items.
    /// Calculating the sum of the values of these items will return the total value of the
    /// attribute.
    #[must_use]
    pub const fn applied(&self) -> &Vec<BonusEntry> {
        &self.applied
    }

    /// Returns the overwritten [`BonusEntry`] items. Overwritten bonuses happen when there is a
    /// larger bonus of the same [`BonusType`] in the [`Breakdowns`].
    #[must_use]
    pub const fn overwritten(&self) -> &Vec<BonusEntry> {
        &self.overwritten
    }

    /// Returns disabled [`BonusEntry`] items. Disabled bonuses are bonuses whose [`Condition`]
    /// evaluates to false.
    ///
    /// [`Condition`]: crate::bonus::Condition
    #[must_use]
    pub const fn disabled(&self) -> &Vec<BonusEntry> {
        &self.disabled
    }

    /// Returns the snapshotted / calculated value of the attribute.
    #[must_use]
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
    pub fn add_breakdown(&mut self, attribute: Attribute) {
        let breakdowns = self.build_breakdowns(&attribute);
        self.cache.breakdowns.insert(attribute, breakdowns);
    }

    /// Returns a reference to the breakdown if it exists for the given attribute.
    #[must_use]
    pub fn get_breakdown(&self, attribute: &Attribute) -> Option<&AttributeBreakdown> {
        self.cache.breakdowns.get(attribute)
    }

    /// Removes and clears all attributes from being cached in breakdowns
    pub fn clear_breakdown(&mut self) {
        self.cache.breakdowns.clear();
    }

    /// Removes an attribute's breakdown from being cached
    pub fn remove_breakdown(&mut self, attribute: &Attribute) -> Option<AttributeBreakdown> {
        self.cache.breakdowns.remove(attribute)
    }

    pub(super) fn build_breakdowns(&mut self, attribute: &Attribute) -> AttributeBreakdown {
        let value = self.evaluate_attribute(attribute);

        let mut breakdown = AttributeBreakdown {
            applied: Vec::new(),
            overwritten: Vec::new(),
            disabled: Vec::new(),
            value,
        };

        let mut applied: HashMap<BonusType, BonusEntry> = HashMap::new();

        let bonuses = self.bonuses.get(attribute).cloned().unwrap_or_default();

        for bonus in bonuses {
            let value = self.evaluate_value(bonus.value());
            let condition = self.evaluate_condition(bonus.condition());

            let entry = BonusEntry { bonus, value };

            if condition {
                match entry.bonus.bonus_type() {
                    BonusType::Stacking => breakdown.applied.push(entry),
                    bonus_type => {
                        if let Some(existing) = applied.remove(bonus_type) {
                            if existing.value >= value {
                                applied.insert(*bonus_type, existing);
                                breakdown.overwritten.push(entry);
                            } else {
                                applied.insert(*bonus_type, entry);
                                breakdown.overwritten.push(existing);
                            }
                        } else {
                            applied.insert(*bonus_type, entry);
                        }
                    }
                }
            } else {
                breakdown.disabled.push(entry);
            }
        }

        breakdown.applied.extend(applied.into_values());

        breakdown
    }
}
//     pub fn get_cached_breakdowns(&self, attribute: &Attribute) -> Option<AttributeBreakdown> {
//         let value = self.cache.attribute.get(attribute)?;
//
//         let mut breakdown = AttributeBreakdown {
//             applied: Vec::new(),
//             overwritten: Vec::new(),
//             disabled: Vec::new(),
//             value,
//         };
//
//         let mut applied: HashMap<BonusType, BonusEntry<'_>> = HashMap::new();
//
//         for bonus in self.bonuses.get(attribute)? {
//             let value = match bonus.value() {
//                 Value::Const(val) => val,
//                 other => self.get_value(other)?,
//             };
//
//             let condition = bonus
//                 .condition()
//                 .map_or(Some(true), |condition| match condition {
//                     Condition::Constant(value) => Some(*value),
//                     condition => self.get_condition(condition),
//                 })?;
//
//             let entry = BonusEntry { bonus, value };
//
//         }
//
//     }
//
//     // pub fn preload_breakdowns(&mut self, attribute: Attribute) {
//     //     let _ = self.evaluate_value(&Value::Attribute(attribute));
//     // }
//     //
//     // pub fn get_breakdowns(&self, attribute: &Attribute) -> Option<AttributeBreakdown> {
//     // }
// }
