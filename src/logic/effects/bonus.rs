use crate::logic::attribute::Attribute;

use super::{BonusType, Condition, Source};

pub struct Bonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: f32,
    condition: Option<Vec<Condition>>,
    source: Source,
}

impl Bonus {
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: f32,
        condition: Option<Vec<Condition>>,
        source: Source
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            condition,
            source
        }
    }

    pub fn get_attribute(&self) -> &Attribute {
        &self.attribute
    }

    pub fn get_bonus_type(&self) -> &BonusType {
        &self.bonus_type
    }

    pub fn get_value(&self) -> &f32 {
        &self.value
    }

    pub fn get_condition(&self) -> &Option<Vec<Condition>> {
        &self.condition
    }
}
