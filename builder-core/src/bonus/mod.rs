mod bonus_source;
pub use bonus_source::*;
mod bonus_types;
pub use bonus_types::*;
mod condition;
pub use condition::*;
mod traits;
use itertools::Itertools;
pub use traits::*;

use super::attribute::Attribute;

#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: f32,
    source: BonusSource,
    conditions: Vec<Condition>,
}

impl ToString for Bonus {
    fn to_string(&self) -> String {
        if self.conditions.len() == 0 {
            format!(
                "{} {} bonus to {}",
                self.value,
                self.bonus_type.to_string(),
                self.attribute.to_string()
            )
        } else {
            format!(
                "{} {} bonus to {} when {}",
                self.value,
                self.bonus_type.to_string(),
                self.attribute.to_string(),
                self.conditions.iter().map(Condition::to_string).join(", ")
            )
        }
    }
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

    pub fn dummy(source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Dummy(),
            bonus_type: BonusType::Stacking,
            value: 0f32,
            source,
            conditions: Vec::new()
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
