mod bonus_source;
pub use bonus_source::*;
mod bonus_types;
pub use bonus_types::*;
mod condition;
pub use condition::*;
mod traits;
pub use traits::*;

use super::attribute::Attribute;

#[derive(PartialEq, Clone)]
pub struct Bonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: f32,
    source: BonusSource,
    conditions: Vec<Condition>,
}

impl Bonus {
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: f32,
        source: BonusSource,
        conditions: Option<Vec<Condition>>,
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            source,
            conditions: conditions.unwrap_or(Vec::new()),
        }
    }

    pub fn get_attribute(&self) -> Attribute {
        self.attribute
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_bonus_type(&self) -> BonusType {
        self.bonus_type
    }

    pub fn get_source(&self) -> BonusSource {
        self.source
    }

    pub fn get_conditions(&self) -> Vec<Condition> {
        self.conditions.clone()
    }
}
