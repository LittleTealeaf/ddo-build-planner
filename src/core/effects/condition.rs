use crate::core::attribute::Attribute;

pub enum Condition {
    FlagAttribute(Attribute),
    MinAttribute(Attribute, f32),
    MaxAttribute(Attribute, f32),
}
