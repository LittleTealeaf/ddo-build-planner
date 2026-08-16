use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
    player_stats::calc::StatsCalculator,
};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct AttributeBreakdown {
    attribute: Attribute,
    base: BreakdownSnapshot,
    snapshots: im::HashMap<u32, BreakdownSnapshot>,
}

impl AttributeBreakdown {
    pub fn snapshot(&self, snapshot: u32) -> Option<&BreakdownSnapshot> {
        self.snapshots.get(&snapshot)
    }
}

impl From<AttributeBreakdown> for Attribute {
    fn from(value: AttributeBreakdown) -> Self {
        value.attribute
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct BreakdownSnapshot {
    value: Decimal,
    applied_bonuses: Vec<CalculatedBonus>,
    overwritten_bonuses: Vec<CalculatedBonus>,
    disabled_bonuses: Vec<CalculatedBonus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, CopyGetters)]
pub struct CalculatedBonus {
    #[getset(get = "pub")]
    value: Decimal,
    #[getset(get_copy = "pub")]
    enabled: bool,
    #[getset(get = "pub")]
    bonus: Bonus,
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
            if !self.get_condition(bonus.show_condition(), snapshot) {
                return None;
            }

            Some(CalculatedBonus {
                value: self.get_value(bonus.value(), snapshot),
                enabled: self.get_condition(bonus.condition(), snapshot),
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
                        .entry(other)
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
            applied_bonuses: applied,
            overwritten_bonuses: overwritten,
            disabled_bonuses: disabled,
        }
    }
}
