//! A Bonus is an individual bonus to an attribute, increasing or decreasing it by a certain amount.
mod bonus_type;
mod condition;
mod source;
mod traits;
mod value;

use crate::attribute::Attribute;

pub use bonus_type::*;
pub use condition::*;
pub use source::*;
pub use traits::*;
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
            condition: None,
        }
    }

    pub fn get_attribute(&self) -> Attribute {
        self.attribute
    }

    pub fn get_type(&self) -> BonusType {
        self.bonus_type
    }

    pub fn get_value(&self) -> BonusValue {
        self.value
    }

    pub fn get_source(&self) -> BonusSource {
        self.source
    }

    pub fn get_condition(&self) -> Option<Condition> {
        self.condition.clone()
    }

    pub fn clone_into_attribute(&self, attribute: Attribute) -> Bonus {
        Bonus {
            attribute,
            bonus_type: self.bonus_type,
            value: self.value,
            source: self.source,
            condition: self.condition.clone(),
        }
    }

    pub fn get_dependencies(&self) -> Option<Vec<Attribute>> {
        let condition_deps = self
            .condition
            .as_ref()
            .map(Condition::get_dependencies);

        let value_deps = self.value.get_dependencies();

        if let Some(mut cond_deps) = condition_deps {
            if let Some(mut val_deps) = value_deps {
                cond_deps.append(&mut val_deps);
            }
            Some(cond_deps)
        } else {
            value_deps
        }
    }
}
