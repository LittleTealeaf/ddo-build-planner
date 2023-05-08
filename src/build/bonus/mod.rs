use super::attribute::Attribute;

mod bonuses;
pub use bonuses::*;
mod condition;
pub use condition::*;
mod source;
pub use source::*;
mod types;
pub use types::*;

#[derive(Clone)]
pub struct Bonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: f32,
    source: Source,
    condition: Vec<Condition>,
}

impl Bonus {
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: f32,
        source: Source,
        condition: Option<Vec<Condition>>,
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            source,
            condition: condition.unwrap_or(vec![]),
        }
    }

    pub fn get_attribute(&self) -> Attribute {
        self.attribute
    }

    pub fn get_bonus_type(&self) -> BonusType {
        self.bonus_type
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_source(&self) -> Source {
        self.source
    }

    pub fn get_condition(&self) -> Vec<Condition> {
        self.condition.clone()
    }
}
