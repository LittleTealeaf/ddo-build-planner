use crate::logic::attribute::Attribute;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Condition {
    Has(Attribute),
    NotHave(Attribute),
    Max(Attribute, f32),
    Min(Attribute, f32),
    Eq(Attribute, f32),
    NotEq(Attribute, f32),
}

impl ToString for Condition {
    fn to_string(&self) -> String {
        match self {
            Condition::Has(attribute) => format!("Has {}", attribute.to_string()),
            Condition::NotHave(attribute) => format!("Does not have {}", attribute.to_string()),
            Condition::Max(attribute, value) => {
                format!("{} is at most {}", attribute.to_string(), value)
            }
            Condition::Min(attribute, value) => {
                format!("{} is at least {}", attribute.to_string(), value)
            }
            Condition::Eq(attribute, value) => format!("{} is {}", attribute.to_string(), value),
            Condition::NotEq(attribute, value) => {
                format!("{} is not {}", attribute.to_string(), value)
            }
        }
    }
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Condition::Has(attr) => {
                if let Condition::Has(other_attr) = other {
                    attr.eq(other_attr)
                } else {
                    false
                }
            }
            Condition::NotHave(attr) => {
                if let Condition::NotHave(other_attr) = other {
                    attr.eq(other_attr)
                } else {
                    false
                }
            }
            Condition::Max(attr, val) => {
                if let Condition::Max(other_attr, other_val) = other {
                    attr.eq(other_attr) && val == other_val
                } else {
                    false
                }
            }
            Condition::Min(attr, val) => {
                if let Condition::Min(other_attr, other_val) = other {
                    attr.eq(other_attr) && val == other_val
                } else {
                    false
                }
            }
            Condition::Eq(attr, val) => {
                if let Condition::Eq(other_attr, other_val) = other {
                    attr.eq(other_attr) && val == other_val
                } else {
                    false
                }
            }
            Condition::NotEq(attr, val) => {
                if let Condition::NotEq(other_attr, other_val) = other {
                    attr.eq(other_attr) && val == other_val
                } else {
                    false
                }
            }
        }
    }
}
