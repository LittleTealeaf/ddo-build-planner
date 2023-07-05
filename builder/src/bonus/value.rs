use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies};

use super::Condition;

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
    /// If [`Condition`] then [`Value`] else [`Value`]
    If(Box<Condition>, Box<Value>, Box<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => value.fmt(f),
            Self::Attribute(attr) => attr.fmt(f),
            Self::Sum(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;
                }

                for val in iter {
                    write!(f, " + {val}")?;
                }

                write!(f, ")")
            }
            Self::Product(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, " * {val}")?;
                    }
                }

                write!(f, ")")
            }
            Self::Min(vals) => {
                write!(f, "Min(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, ", {val}")?;
                    }
                }

                write!(f, ")")
            }
            Self::Max(vals) => {
                write!(f, "Max(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, ", {val}")?;
                    }
                }

                write!(f, ")")
            }
            Self::Floor(val) => write!(f, "Floor({val})"),
            Self::If(cond, if_true, if_false) => {
                write!(f, "If ({cond}) then {if_true} else {if_false}")
            }
        }
    }
}

impl AttributeDependencies for Value {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        match self {
            Self::Value(_) => false,
            Self::Attribute(attr) => attribute.eq(attr),
            Self::Min(vals) | Self::Max(vals) | Self::Product(vals) | Self::Sum(vals) => {
                vals.iter().any(|val| val.has_attr_dependency(attribute))
            }
            Self::Floor(val) => val.has_attr_dependency(attribute),
            Self::If(cond, if_true, if_false) => {
                cond.has_attr_dependency(attribute)
                    || if_true.has_attr_dependency(attribute)
                    || if_false.has_attr_dependency(attribute)
            }
        }
    }

    fn include_attr_dependency(&self, set: &mut im::OrdSet<Attribute>) {
        match self {
            Self::Value(_) => {}
            Self::Attribute(attr) => {
                set.insert(*attr);
            }
            Self::Min(vals) | Self::Max(vals) | Self::Product(vals) | Self::Sum(vals) => {
                for val in vals {
                    val.include_attr_dependency(set);
                }
            }
            Self::Floor(val) => val.include_attr_dependency(set),
            Self::If(cond, if_true, if_false) => {
                cond.include_attr_dependency(set);
                if_true.include_attr_dependency(set);
                if_false.include_attr_dependency(set);
            }
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

impl From<Attribute> for Value {
    fn from(value: Attribute) -> Self {
        Self::Attribute(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add tests for dependencies

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

    mod dependencies {
        use super::*;

        mod has_dependency {
            use super::*;

            #[test]
            fn value() {
                let value = Value::Value(10f32);

                assert!(!value.has_attr_dependency(Attribute::Debug(0)));
            }

            #[test]
            fn attribute() {
                let value = Value::Attribute(Attribute::Debug(0));

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn sum() {
                let value = Value::Sum(vec![Attribute::Debug(0).into(), 10f32.into()]);

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn product() {
                let value = Value::Product(vec![Attribute::Debug(0).into(), 10f32.into()]);

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn min() {
                let value = Value::Min(vec![Attribute::Debug(0).into(), 10f32.into()]);

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn max() {
                let value =
                    Value::Max(vec![Attribute::Debug(0).into(), Attribute::Debug(1).into()]);

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(value.has_attr_dependency(Attribute::Debug(1)));
                assert!(!value.has_attr_dependency(Attribute::Debug(2)));
            }

            #[test]
            fn floor() {
                let value = Value::Floor(Value::Attribute(Attribute::Debug(0)).into());

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn if_value() {
                let value = Value::If(
                    Condition::GreaterThan(Attribute::Debug(0).into(), 1f32.into()).into(),
                    Value::from(Attribute::Debug(1)).into(),
                    Value::from(Attribute::Debug(2)).into(),
                );

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(value.has_attr_dependency(Attribute::Debug(1)));
                assert!(value.has_attr_dependency(Attribute::Debug(2)));
                assert!(!value.has_attr_dependency(Attribute::Debug(3)));
            }
        }

        mod include_dependencies {
            use super::*;

            #[test]
            fn value() {
                let value = Value::Value(10f32);
                let deps = value.get_attr_dependencies();

                assert!(!deps.contains(&Attribute::Debug(0)));
            }

            #[test]
            fn attribute() {
                let value = Value::Attribute(Attribute::Debug(0));

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn sum() {
                let value = Value::Sum(vec![Attribute::Debug(0).into(), 10f32.into()]);

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn product() {
                let value = Value::Product(vec![Attribute::Debug(0).into(), 10f32.into()]);

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn min() {
                let value = Value::Min(vec![Attribute::Debug(0).into(), 10f32.into()]);

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn max() {
                let value =
                    Value::Max(vec![Attribute::Debug(0).into(), Attribute::Debug(1).into()]);

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(deps.contains(&Attribute::Debug(1)));
                assert!(!deps.contains(&Attribute::Debug(2)));
            }

            #[test]
            fn floor() {
                let value = Value::Floor(Value::Attribute(Attribute::Debug(0)).into());

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn if_value() {
                let value = Value::If(
                    Condition::GreaterThan(Attribute::Debug(0).into(), 1f32.into()).into(),
                    Value::from(Attribute::Debug(1)).into(),
                    Value::from(Attribute::Debug(2)).into(),
                );

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(deps.contains(&Attribute::Debug(1)));
                assert!(deps.contains(&Attribute::Debug(2)));
                assert!(!deps.contains(&Attribute::Debug(3)));
            }
        }
    }
}
