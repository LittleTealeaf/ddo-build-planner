use crate::attribute::Attribute;

#[derive(Clone)]
pub enum Condition {
    Has(Attribute),
    NotHave(Attribute),
    Max(Attribute, f32),
    Min(Attribute, f32),
    Eq(Attribute, f32),
    NotEq(Attribute, f32),
    Any(Vec<Condition>),
    All(Vec<Condition>),
}
