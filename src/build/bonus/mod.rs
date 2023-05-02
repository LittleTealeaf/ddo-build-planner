use self::{condition::Condition, types::BonusType, source::Source};

use super::attribute::Attribute;

pub mod condition;
pub mod source;
pub mod types;
pub mod bonuses;

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
}
