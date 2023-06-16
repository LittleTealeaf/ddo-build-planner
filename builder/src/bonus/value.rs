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
}

impl BonusValue {
    /// Returns any dependencies associated with the value.
    ///
    /// In short terms: If the [`BonusValue`] has an [`Attribute`] in it, then this returns a
    /// [`Vec`] with all attributes included.
    pub fn get_dependencies(&self) -> Option<Vec<Attribute>> {
        match self {
            Self::Attribute(attribute) => Some(vec![*attribute]),
            Self::Sum(vals) | Self::Product(vals) => Some(
                vals.iter()
                    .filter_map(BonusValue::get_dependencies)
                    .flatten()
                    .collect(),
            ),
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
