use crate::attribute::Attribute;

#[derive(Copy, Clone)]
pub enum Condition {
    Has(Attribute),
    NotHave(Attribute),
    Max(Attribute, f32),
    Min(Attribute, f32),
    Eq(Attribute, f32),
    NotEq(Attribute, f32),
}
