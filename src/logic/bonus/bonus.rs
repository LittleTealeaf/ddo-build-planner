use crate::logic::attribute::Attribute;

use super::{BonusSource, BonusType, Condition};

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
}
