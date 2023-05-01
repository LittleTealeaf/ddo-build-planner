use crate::core::attribute::Attribute;

use super::{BonusType, Condition};

pub struct Bonus {
    pub attribute: Attribute,
    pub bonus_type: BonusType,
    pub value: f32,
    pub condition: Option<Vec<Condition>>,
}
