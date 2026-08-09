use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusCondition, BonusSource, BonusType, BonusValue},
    player_stats::calc::StatsCalculator,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeBreakdown {
    attribute: Attribute,
    base: BreakdownSnapshot,
    snapshots: im::HashMap<u32, BreakdownSnapshot>,
}

impl AttributeBreakdown {
    #[must_use]
    pub const fn attribute(&self) -> &Attribute {
        &self.attribute
    }

    #[must_use]
    pub const fn base(&self) -> &BreakdownSnapshot {
        &self.base
    }

    pub fn snapshot(&self, snapshot: u32) -> Option<&BreakdownSnapshot> {
        self.snapshots.get(&snapshot)
    }
}

impl From<AttributeBreakdown> for Attribute {
    fn from(value: AttributeBreakdown) -> Self {
        value.attribute
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakdownSnapshot {
    value: Decimal,
    applied: Vec<CalculatedBonus>,
    overwritten: Vec<CalculatedBonus>,
    disabled: Vec<CalculatedBonus>,
}

impl BreakdownSnapshot {
    #[must_use]
    pub const fn value(&self) -> Decimal {
        self.value
    }

    #[must_use]
    pub const fn applied_bonuses(&self) -> &Vec<CalculatedBonus> {
        &self.applied
    }

    #[must_use]
    pub const fn overwritten_bonuses(&self) -> &Vec<CalculatedBonus> {
        &self.overwritten
    }

    #[must_use]
    pub const fn disabled_bonuses(&self) -> &Vec<CalculatedBonus> {
        &self.disabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculatedBonus {
    value: Decimal,
    enabled: bool,
    bonus: Bonus,
}

impl CalculatedBonus {
    pub const fn value(&self) -> Decimal {
        self.value
    }

    pub const fn bonus_value(&self) -> &BonusValue {
        self.bonus.value()
    }

    pub const fn source(&self) -> &BonusSource {
        self.bonus.source()
    }

    pub const fn condition(&self) -> Option<&BonusCondition> {
        self.bonus.condition().as_ref()
    }

    pub const fn bonus_type(&self) -> &BonusType {
        self.bonus.bonus_type()
    }

    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl StatsCalculator<'_> {
    pub fn update_breakdown(&mut self, attribute: Attribute) {
        let breakdown = self.build_breakdowns(attribute.clone());
        self.cache.breakdowns.insert(attribute, breakdown);
    }

    fn build_breakdowns(&mut self, attribute: Attribute) -> AttributeBreakdown {
        let base = self.build_snapshot_breakdown(&attribute, None);
        let snapshots = self.bonuses.get_snapshots(&attribute);

        let snapshots = snapshots
            .into_iter()
            .map(|snapshot| {
                (
                    snapshot,
                    self.build_snapshot_breakdown(&attribute, Some(snapshot)),
                )
            })
            .collect();

        AttributeBreakdown {
            attribute,
            base,
            snapshots,
        }
    }

    fn build_snapshot_breakdown(
        &mut self,
        attribute: &Attribute,
        snapshot: Option<u32>,
    ) -> BreakdownSnapshot {
        let value = self.evaluate_attribute(attribute.clone(), snapshot);

        let bonuses = self.bonuses.get_snapshot_bonuses(attribute, snapshot);
        let calculated_bonuses = bonuses.filter_map(|bonus| {
            if let Some(show_condition) = bonus.show_condition() {
                if !self.get_condition(show_condition, snapshot) {
                    return None;
                }
            }

            Some(CalculatedBonus {
                value: self.get_value(bonus.value(), snapshot),
                enabled: bonus
                    .condition()
                    .as_ref()
                    .is_none_or(|condition| self.get_condition(condition, snapshot)),
                bonus: bonus.clone(),
            })
        });

        let mut applied = Vec::new();
        let mut typed_applied = HashMap::<BonusType, CalculatedBonus>::new();
        let mut overwritten = Vec::new();
        let mut disabled = Vec::new();

        for bonus in calculated_bonuses {
            if !bonus.enabled {
                disabled.push(bonus);
                continue;
            }

            match bonus.bonus.bonus_type() {
                BonusType::Stacking => applied.push(bonus),
                other => {
                    typed_applied
                        .entry(*other)
                        .and_modify(|entry| {
                            if entry.value < bonus.value {
                                let mut b = bonus.clone();
                                core::mem::swap(entry, &mut b);
                                overwritten.push(b);
                            }
                        })
                        .or_insert_with(|| bonus.clone());
                }
            }
        }

        applied.extend(typed_applied.into_values());

        BreakdownSnapshot {
            value,
            applied,
            overwritten,
            disabled,
        }
    }
}
