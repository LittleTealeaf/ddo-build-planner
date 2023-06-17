use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

use super::{Bonus, BonusSource, BonusType, BonusValue, Condition};

/// A bonus without a [`BonusSource`], used in items as the source is tied to the slot, not the
/// item.
///
/// [`BonusSource`]: crate::bonus::BonusSource
#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateBonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: BonusValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Condition>,
}

impl TemplateBonus {
    /// Creates a new TemplateBonus with all attributes except for the source
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: BonusValue,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            condition,
        }
    }

    /// Converts the current TemplateBonus to a Bonus
    pub fn to_bonus(self, source: BonusSource) -> Bonus {
        Bonus::new(
            self.attribute,
            self.bonus_type,
            self.value,
            source,
            self.condition,
        )
    }
}
