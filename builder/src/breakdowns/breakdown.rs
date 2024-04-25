use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType, Condition, Value},
};

use super::Breakdowns;

/// Contains a reference to the bonus as well as the final value of that bonus.
#[derive(Debug)]
pub struct BonusEntry<'a> {
    bonus: &'a Bonus,
    value: &'a Decimal,
}

impl<'a> BonusEntry<'a> {
    /// Returns a reference to the bonus for this object
    #[must_use]
    pub const fn bonus(&self) -> &'a Bonus {
        self.bonus
    }

    /// Returns a reference to the calculated value for this bonus
    #[must_use]
    pub const fn value(&self) -> &'a Decimal {
        self.value
    }
}

/// Describes the breakdowns used when calculating a given attribute.
#[derive(Debug)]
pub struct AttributeBreakdown<'a> {
    applied: Vec<BonusEntry<'a>>,
    overwritten: Vec<BonusEntry<'a>>,
    disabled: Vec<BonusEntry<'a>>,
    value: Decimal,
}

impl<'a> AttributeBreakdown<'a> {
    /// Returns a vector of bonus entries that are actively applied and contribute towards the
    /// final value
    #[must_use]
    pub const fn applied(&self) -> &Vec<BonusEntry<'a>> {
        &self.applied
    }

    /// Returns a vector of bonus entries that are overwritten, meaning that there are bonuses of
    /// the same [`BonusType`] that have higher bonus values
    #[must_use]
    pub const fn overwritten(&self) -> &Vec<BonusEntry<'a>> {
        &self.overwritten
    }

    /// Returns a vector of the disabled bonuses, or bonuses whose conditions are not fulfilled, so
    /// their bonuses do not contribute towards the final value
    #[must_use]
    pub const fn disabled(&self) -> &Vec<BonusEntry<'a>> {
        &self.disabled
    }

    /// Returns the final calculated value
    #[must_use]
    pub const fn value(&self) -> &Decimal {
        &self.value
    }
}

impl Breakdowns {
    /// Returns the bonus breakdowns for a particular attribute in the breakdown
    ///
    /// # Panics
    /// Panics will not happen unless the values in the caches are removed during execution of this
    /// function
    pub fn get_breakdowns(&mut self, attribute: &Attribute) -> Option<AttributeBreakdown> {
        let value = self.calculate_attribute(attribute)?;

        let mut breakdown = AttributeBreakdown {
            applied: Vec::new(),
            overwritten: Vec::new(),
            disabled: Vec::new(),
            value,
        };

        let mut applied: HashMap<BonusType, BonusEntry<'_>> = HashMap::new();

        for bonus in self.bonuses.get(attribute)? {
            let value = match bonus.value() {
                Value::Const(val) => val,
                other => self
                    .value_cache
                    .get(other)
                    .unwrap_or_else(|| panic!("Expected Value to be Cached: {value}")),
            };

            let condition = bonus.condition().map_or(true, |condition| match condition {
                Condition::Constant(value) => *value,
                condition => *self
                    .condition_cache
                    .get(condition)
                    .unwrap_or_else(|| panic!("Expected Condition to be Cached: {condition}")),
            });

            let entry = BonusEntry { bonus, value };

            if condition {
                match bonus.bonus_type() {
                    BonusType::Stacking => breakdown.applied.push(BonusEntry { bonus, value }),
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

        Some(breakdown)
    }
}
