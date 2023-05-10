use crate::logic::attribute::Attribute;

#[derive(Clone, Copy)]
pub enum Condition {
    Has(Attribute),
    Max(Attribute, f32),
    Min(Attribute, f32),
    Eq(Attribute, f32),
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
                if let Condition::Min(other_attr, other_val) = other {
                    attr.eq(other_attr) && val == other_val
                } else {
                    false
                }
            }
        }
    }
}
