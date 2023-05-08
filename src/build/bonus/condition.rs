use crate::build::attribute::{self, Attribute, Flag};

#[derive(Clone, Copy)]
pub enum Condition {
    Has(Attribute),
    Minimum(Attribute, f32),
    Maximum(Attribute, f32),
    Equals(Attribute, f32),
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Condition::Has(attribute) => {
                if let Condition::Has(other_attribute) = other {
                    attribute.eq(other_attribute)
                } else {
                    false
                }
            }
            Condition::Minimum(attribute, value) => {
                if let Condition::Minimum(other_attribute, other_value) = other {
                    attribute.eq(other_attribute) && value == other_value
                } else {
                    false
                }
            }
            Condition::Maximum(attribute, value) => {
                if let Condition::Maximum(other_attribute, other_value) = other {
                    attribute.eq(other_attribute) && value == other_value
                } else {
                    false
                }
            }
            Condition::Equals(attribute, value) => {
                if let Condition::Equals(other_attribute, other_value) = other {
                    attribute.eq(other_attribute) && value == other_value
                } else {
                    false
                }
            }
        }
    }
}

impl Condition {
    pub fn into_vec(self) -> Vec<Condition> {
        vec![self]
    }
}
