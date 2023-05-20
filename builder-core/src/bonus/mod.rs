mod bonus_source;
pub use bonus_source::*;
mod bonus_types;
pub use bonus_types::*;
mod condition;
pub use condition::*;
mod traits;
use itertools::Itertools;
pub use traits::*;

use crate::attribute::{Flag, Toggle, Immunity};

use super::attribute::Attribute;

#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Bonus {
    #[serde(rename = "atr")]
    attribute: Attribute,
    #[serde(rename = "ty")]
    bonus_type: BonusType,
    #[serde(rename = "val")]
    value: f32,
    #[serde(rename = "src")]
    source: BonusSource,
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<Condition>>,
}

impl ToString for Bonus {
    fn to_string(&self) -> String {
        if let Some(conditions) = &self.conditions {
            format!(
                "{} {} bonus to {} when {}",
                self.value,
                self.bonus_type.to_string(),
                self.attribute.to_string(),
                conditions.iter().map(Condition::to_string).join(", ")
            )
        } else {
            format!(
                "{} {} bonus to {}",
                self.value,
                self.bonus_type.to_string(),
                self.attribute.to_string()
            )
        }
    }
}


impl Bonus {
    #[inline(always)]
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
            conditions,
        }
    }
    #[inline(always)]
    pub fn dummy(source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Dummy(),
            bonus_type: BonusType::Stacking,
            value: 0f32,
            source,
            conditions: None,
        }
    }

    #[inline(always)]
    pub fn flag(flag: Flag, source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Flag(flag),
            bonus_type: BonusType::Stacking,
            value: 1f32,
            source,
            conditions: None,
        }
    }

    #[inline(always)]
    pub fn toggle(toggle: Toggle, source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Toggle(toggle),
            bonus_type: BonusType::Stacking,
            value: 1f32,
            source,
            conditions: None,
        }
    }

    #[inline(always)]
    pub fn immunity(immunity: Immunity, source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Immunity(immunity),
            bonus_type: BonusType::Stacking,
            value: 1f32,
            source,
            conditions: None
        }
    }

    #[inline(always)]
    pub fn get_attribute(&self) -> Attribute {
        self.attribute
    }

    #[inline(always)]
    pub fn get_value(&self) -> f32 {
        self.value
    }

    #[inline(always)]
    pub fn get_bonus_type(&self) -> BonusType {
        self.bonus_type
    }

    #[inline(always)]
    pub fn get_source(&self) -> BonusSource {
        self.source
    }

    #[inline(always)]
    pub fn get_conditions(&self) -> Option<Vec<Condition>> {
        self.conditions.clone()
    }
}
