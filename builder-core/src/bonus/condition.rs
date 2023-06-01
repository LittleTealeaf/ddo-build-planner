use crate::attribute::Attribute;

/// Describes an attribute-based condition that must be met for a bonus to be included.
#[derive(Clone, Debug)]
pub enum Condition {
    /// Requires that an attribute has an above 0 value
    Has(Attribute),
    /// Requires that an attribute is either zero or below
    NotHave(Attribute),
    /// Requires that an attribute has at most some value
    Max(Attribute, f32),
    /// Requires that an attribute has at least some value
    Min(Attribute, f32),
    /// Requires that an attribute is exactly some value
    Eq(Attribute, f32),
    /// Requires that an attribute is not equal to some value
    NotEq(Attribute, f32),
    /// Requires any of the provided conditions
    Any(Vec<Condition>),
    /// Requires all of the provided conditions
    All(Vec<Condition>),
}

impl Condition {
    // Returns any dependant condition
    pub fn get_dependencies(&self) -> Vec<Attribute> {
        match self {
            Condition::Has(attr)
            | Condition::NotHave(attr)
            | Condition::Max(attr, _)
            | Condition::Min(attr, _)
            | Condition::Eq(attr, _)
            | Condition::NotEq(attr, _) => vec![*attr],
            Condition::Any(conds) | Condition::All(conds) => {
                conds.iter().flat_map(Condition::get_dependencies).collect()
            }
        }
    }

    pub fn has_any(&self, attributes: Vec<Attribute>) -> Condition {
        Condition::Any(attributes.into_iter().map(Condition::Has).collect())
    }

    pub fn has_all(&self, attributes: Vec<Attribute>) -> Condition {
        Condition::All(attributes.into_iter().map(Condition::Has).collect())
    }
}
