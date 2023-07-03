use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies};

/// Represents a value of a [`Bonus`]
///
/// [`Bonus`]: crate::bonus::Bonus
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Value {
    /// Just a simple [`f32`] value.
    Value(f32),
    /// Copy the total value of some [`Attribute`].
    Attribute(Attribute),
    /// Sums each of the values
    Sum(Vec<Value>),
    /// Multiplies each of the values
    Product(Vec<Value>),
    /// Returns the minimum value from the set
    Min(Vec<Value>),
    /// Returns the maximum value from the set
    Max(Vec<Value>),
    /// Floors the inner value to a whole number
    Floor(Box<Value>),
}

impl Value {
    /// Returns any dependencies associated with the value.
    ///
    /// In short terms: If the [`BonusValue`] has an [`Attribute`] in it, then this returns a
    /// [`Vec`] with all attributes included.
    pub fn get_dependencies(&self) -> Option<Vec<Attribute>> {
        match self {
            Self::Attribute(attribute) => Some(vec![*attribute]),
            Self::Sum(vals) | Self::Product(vals) | Self::Min(vals) | Self::Max(vals) => Some(
                vals.iter()
                    .filter_map(Value::get_dependencies)
                    .flatten()
                    .collect(),
            ),
            Self::Floor(val) => val.get_dependencies(),
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Value(value) => value.fmt(f),
            Value::Attribute(attr) => attr.fmt(f),
            Value::Sum(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;
                }

                for val in iter {
                    write!(f, " + {}", val)?;
                }

                write!(f, ")")
            }
            Value::Product(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, " * {}", val)?;
                    }
                }

                write!(f, ")")
            }
            Value::Min(vals) => {
                write!(f, "Min(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, ", {}", val)?;
                    }
                }

                write!(f, ")")
            }
            Value::Max(vals) => {
                write!(f, "Max(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, ", {}", val)?;
                    }
                }

                write!(f, ")")
            }
            Value::Floor(val) => write!(f, "Floor({})", val),
        }
    }
}

impl AttributeDependencies for Value {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        match self {
            Value::Value(_) => false,
            Value::Attribute(attr) => attribute.eq(attr),
            Value::Min(vals) | Value::Max(vals) | Value::Product(vals) | Value::Sum(vals) => vals
                .into_iter()
                .any(|val| val.has_attr_dependency(attribute)),
            Value::Floor(val) => val.has_attr_dependency(attribute),
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Value::Value(value)
    }
}

impl From<Attribute> for Value {
    fn from(value: Attribute) -> Self {
        Value::Attribute(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_returns_dependency() {
        let value = Value::Attribute(Attribute::Debug(3));
        let dependencies = value.get_dependencies();

        assert_eq!(Some(vec![Attribute::Debug(3)]), dependencies);
    }

    #[test]
    fn value_returns_no_dependency() {
        let value = Value::Value(0f32);
        let dependencies = value.get_dependencies();

        assert_eq!(None, dependencies);
    }

    #[test]
    fn sum_returns_dependencies() {
        let value = Value::Sum(vec![
            Value::Attribute(Attribute::Debug(5)),
            Value::Value(3f32),
        ]);
        let dependencies = value.get_dependencies();

        assert_eq!(Some(vec![Attribute::Debug(5)]), dependencies);
    }

    #[test]
    fn product_returns_dependencies() {
        let value = Value::Product(vec![
            Value::Attribute(Attribute::Debug(5)),
            Value::Value(3f32),
        ]);
        let dependencies = value.get_dependencies();

        assert_eq!(Some(vec![Attribute::Debug(5)]), dependencies);
    }

    #[test]
    fn from_attribute() {
        let value = Value::from(Attribute::Debug(4));
        assert_eq!(value, Value::Attribute(Attribute::Debug(4)));
    }

    #[test]
    fn from_value() {
        let value = Value::from(3f32);

        assert!({
            if let Value::Value(val) = value {
                val == 3f32
            } else {
                false
            }
        });
    }
}
