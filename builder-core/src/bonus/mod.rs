//! A Bonus is an individual bonus to an attribute, increasing or decreasing it by a certain amount.
mod bonus_type;
mod condition;
mod source;
mod value;

use crate::attribute::Attribute;

pub use bonus_type::*;
pub use condition::*;
pub use source::*;
pub use value::*;

#[derive(Debug, Clone)]
pub struct Bonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: BonusValue,
    source: BonusSource,
    condition: Option<Condition>,
}

impl Bonus {
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: BonusValue,
        source: BonusSource,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            source,
            condition,
        }
    }

    pub fn dummy(source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Dummy,
            bonus_type: BonusType::Stacking,
            value: 0f32.into(),
            source,
            condition: None
        }
    }

    pub fn get_attribute(&self) -> Attribute {
        self.attribute
    }

    pub fn get_bous_type(&self) -> BonusType {
        self.bonus_type
    }

    pub fn get_bonus_value(&self) -> BonusValue {
        self.value
    }

    pub fn get_source(&self) -> BonusSource {
        self.source
    }

    pub fn get_condition(&self) -> Option<Condition> {
        self.condition.clone()
    }
}
