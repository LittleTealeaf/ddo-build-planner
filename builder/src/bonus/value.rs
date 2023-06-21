use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

/// Represents a value of a [`Bonus`]
///
/// [`Bonus`]: crate::bonus::Bonus
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum BonusValue {
    /// Just a simple [`f32`] value.
    Value(f32),
    /// Copy the total value of some [`Attribute`].
    Attribute(Attribute),
    /// Sums each of the values
    Sum(Vec<BonusValue>),
    /// Multiplies each of the values
    Product(Vec<BonusValue>),
    /// Returns the minimum value from the set
    Min(Vec<BonusValue>),
    /// Returns the maximum value from the set
    Max(Vec<BonusValue>),
    /// Floors the inner value to a whole number
    Floor(Box<BonusValue>),
}

impl BonusValue {
    /// Returns any dependencies associated with the value.
    ///
    /// In short terms: If the [`BonusValue`] has an [`Attribute`] in it, then this returns a
    /// [`Vec`] with all attributes included.
    pub fn get_dependencies(&self) -> Option<Vec<Attribute>> {
        match self {
            Self::Attribute(attribute) => Some(vec![*attribute]),
            Self::Sum(vals) | Self::Product(vals) | Self::Min(vals) | Self::Max(vals) => Some(
                vals.iter()
                    .filter_map(BonusValue::get_dependencies)
                    .flatten()
                    .collect(),
            ),
            Self::Floor(val) => val.get_dependencies(),
            _ => None,
        }
    }
}

impl Display for BonusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BonusValue::Value(value) => value.fmt(f),
            BonusValue::Attribute(attr) => attr.fmt(f),
            BonusValue::Sum(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;
                }

                while let Some(val) = iter.next() {
                    write!(f, " + {}", val)?;
                }

                write!(f, ")")
            }
            BonusValue::Product(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    while let Some(val) = iter.next() {
                        write!(f, " * {}", val)?;
                    }
                }

                write!(f, ")")
            }
            BonusValue::Min(vals) => {
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
            BonusValue::Max(vals) => {
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
            BonusValue::Floor(val) => write!(f, "Floor({})", val),
        }
    }
}

impl From<f32> for BonusValue {
    fn from(value: f32) -> BonusValue {
        BonusValue::Value(value)
    }
}

impl From<Attribute> for BonusValue {
    fn from(value: Attribute) -> Self {
        BonusValue::Attribute(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_returns_dependency() {
        let value = BonusValue::Attribute(Attribute::Debug(3));
        let dependencies = value.get_dependencies();

        assert_eq!(Some(vec![Attribute::Debug(3)]), dependencies);
    }

    #[test]
    fn value_returns_no_dependency() {
        let value = BonusValue::Value(0f32);
        let dependencies = value.get_dependencies();

        assert_eq!(None, dependencies);
    }

    #[test]
    fn sum_returns_dependencies() {
        let value = BonusValue::Sum(vec![
            BonusValue::Attribute(Attribute::Debug(5)),
            BonusValue::Value(3f32),
        ]);
        let dependencies = value.get_dependencies();

        assert_eq!(Some(vec![Attribute::Debug(5)]), dependencies);
    }

    #[test]
    fn product_returns_dependencies() {
        let value = BonusValue::Product(vec![
            BonusValue::Attribute(Attribute::Debug(5)),
            BonusValue::Value(3f32),
        ]);
        let dependencies = value.get_dependencies();

        assert_eq!(Some(vec![Attribute::Debug(5)]), dependencies);
    }

    #[test]
    fn from_attribute() {
        let value = BonusValue::from(Attribute::Debug(4));
        assert_eq!(value, BonusValue::Attribute(Attribute::Debug(4)));
    }

    #[test]
    fn from_value() {
        let value = BonusValue::from(3f32);

        assert!({
            if let BonusValue::Value(val) = value {
                val == 3f32
            } else {
                false
            }
        });
    }
}
