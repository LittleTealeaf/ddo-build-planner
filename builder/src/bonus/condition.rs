use crate::attribute::Attribute;

/// Describes an attribute-based condition that must be met for a bonus to be included.
#[derive(Clone, Debug)]
pub enum Condition {
    /// Negates a condition
    Not(Box<Condition>),
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
    /// Requires that an attribute is greater than another attribute
    GreaterThan(Attribute, Attribute),
    /// Requires that an attribute is less than another attribute
    LessThan(Attribute, Attribute),
    /// Requires that an attribute is equal to another attribute
    EqualTo(Attribute, Attribute),
    /// Requires any of the provided conditions
    Any(Vec<Condition>),
    /// Requires all of the provided conditions
    All(Vec<Condition>),
    /// Requires that not all of the provided conditions are true
    NotAll(Vec<Condition>),
    /// Requires that none of the provided conditions are true
    None(Vec<Condition>),
}

/// Implements different constructors to make building conditions easier.
impl Condition {
    /// Creates a condition that checks that any of the provided attributes are present.
    ///
    ///
    /// Returns a [`Condition::Any`] with a list of [`Condition::Has`] conditions for each of the provided attributes.
    pub fn has_any(attributes: Vec<Attribute>) -> Condition {
        Condition::Any(attributes.into_iter().map(Condition::Has).collect())
    }

    /// Creates a condition that checks that all of the provided attributes are present.
    ///
    /// Returns a [`Condition::All`] with a list of [`Condition::Has`] conditions for each of the provided attributes.
    pub fn has_all(attributes: Vec<Attribute>) -> Condition {
        Condition::All(attributes.into_iter().map(Condition::Has).collect())
    }

    /// Creates a condition that checks that none of the provided attributes are present.
    ///
    /// Returns a [`Condition::All`] with a list of [`Condition::NotHave`] conditions for each of the provided attributes.
    pub fn not_have_any(attributes: Vec<Attribute>) -> Condition {
        Condition::All(attributes.into_iter().map(Condition::NotHave).collect())
    }

    /// Creates a condition that checks that at least one of the provided arguments is not present.
    ///
    /// Returns a [`Condition::Any`] with a list of [`Condition::NotHave`] conditions for each of the provided attributes.
    pub fn not_have_all(attributes: Vec<Attribute>) -> Condition {
        Condition::Any(attributes.into_iter().map(Condition::NotHave).collect())
    }
}

/// Methods that can be called from a condition.
impl Condition {
    /// Returns any dependant condition
    pub fn get_dependencies(&self) -> Vec<Attribute> {
        match self {
            Condition::Not(condition) => condition.get_dependencies(),
            Condition::GreaterThan(a, b) | Condition::LessThan(a, b) | Condition::EqualTo(a, b) => {
                vec![*a, *b]
            }
            Condition::Has(attr)
            | Condition::NotHave(attr)
            | Condition::Max(attr, _)
            | Condition::Min(attr, _)
            | Condition::Eq(attr, _)
            | Condition::NotEq(attr, _) => vec![*attr],
            Condition::Any(conds)
            | Condition::All(conds)
            | Condition::NotAll(conds)
            | Condition::None(conds) => {
                conds.iter().flat_map(Condition::get_dependencies).collect()
            }
        }
    }
}