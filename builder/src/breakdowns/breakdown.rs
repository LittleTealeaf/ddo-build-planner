use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

use super::{Breakdowns, EvalBonus};

#[derive(Debug)]
pub struct BonusEntry<'a> {
    bonus: &'a Bonus,
    value: &'a Decimal,
}

#[derive(Debug)]
pub struct AttributeBreakdown<'a> {
    applied: Vec<BonusEntry<'a>>,
    overwritten: Vec<BonusEntry<'a>>,
    disabled: Vec<BonusEntry<'a>>,
    value: Decimal,
}

impl Breakdowns {
    /// Returns the bonus breakdowns for a particular attribute in the breakdown
    pub fn get_breakdowns<'a>(
        &'a mut self,
        attribute: &Attribute,
    ) -> Option<AttributeBreakdown<'a>> {
        let mut breakdown = AttributeBreakdown {
            applied: Vec::new(),
            overwritten: Vec::new(),
            disabled: Vec::new(),
            value: self.calculate_attribute(*attribute)?,
        };

        // Assumes that calling self.get_attribute will populate all bonus caches

        let bonuses = self
            .bonuses
            .get(attribute)?
            .iter()
            .filter_map(|bonus| Some((bonus, self.bonus_cache.get(bonus)?)));

        let mut applied: HashMap<BonusType, BonusEntry<'_>> = HashMap::new();

        for (bonus, EvalBonus { value, condition }) in bonuses {
            if *condition {
                match bonus.get_type() {
                    BonusType::Stacking => breakdown.applied.push(BonusEntry { bonus, value }),
                    bonus_type => {
                        if let Some(existing) = applied.remove(bonus_type) {
                            let bonus = BonusEntry { bonus, value };
                            let (larger, smaller) = if existing.value > value {
                                (existing, bonus)
                            } else {
                                (bonus, existing)
                            };
                            applied.insert(*bonus_type, larger);
                            breakdown.overwritten.push(smaller);
                        } else {
                            applied.insert(*bonus_type, BonusEntry { bonus, value });
                        }
                    }
                }
            } else {
                breakdown.disabled.push(BonusEntry { bonus, value });
            }
        }

        breakdown.applied.extend(applied.into_values());

        Some(breakdown)
    }
}
