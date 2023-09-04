use std::fmt::Display;

use im::OrdMap;

use super::Dice;

/// Describes a [`Dice`] structure that can be converted into a [`Dice`] when given the placeholder
/// variable values
pub enum DiceFormula<T>
where
    T: Ord + Eq,
{
    /// A fully defined value
    Value(f32),
    /// Placeholder Variable
    Variable(T),
    /// A fully defined dice
    Roll {
        /// The number of dice being rolled
        count: u16,
        /// The nubmer of sides on the dice
        sides: u16,
    },
    /// A dice where the count is a variable
    VariableCountRoll {
        /// The number of dice being rolled
        count: T,
        /// The nubmer of sides on the dice
        sides: u16,
    },
    /// A dice where the size is variable
    VariableSizeRoll {
        /// The number of dice being rolled
        count: u16,
        /// The nubmer of sides on the dice
        sides: T,
    },
    /// A dice where both the count and the size are variable
    VariableRoll {
        /// The number of dice being rolled
        count: T,
        /// The nubmer of sides on the dice
        sides: T,
    },
    /// The sum of multiple Dice Formulae
    Sum(Vec<DiceFormula<T>>),
    /// The product of multiple dice formulae
    Product(Vec<DiceFormula<T>>),
}

impl<T> DiceFormula<T>
where
    T: Ord + Eq,
{
    /// Attempts to convert a [`DiceFormula`] into a [`Dice`] object, provided a set of variables
    /// to insert into the placeholder locations
    pub fn to_dice(self, variables: &OrdMap<T, f32>) -> Result<Dice, ConvertDiceError<T>> {
        match self {
            Self::Value(value) => Ok(Dice::Value(value)),
            Self::Variable(variable) => Ok(Dice::Value(
                variables
                    .get(&variable)
                    .map(|var| *var)
                    .ok_or(ConvertDiceError::MissingVariable(variable))?,
            )),
            Self::Roll { count, sides } => Ok(Dice::Roll { count, sides }),
            Self::VariableCountRoll { count, sides } => Ok(Dice::Roll {
                count: variables
                    .get(&count)
                    .map(|var| *var)
                    .ok_or(ConvertDiceError::MissingVariable(count))?
                    .floor() as u16,
                sides,
            }),
            Self::VariableSizeRoll { count, sides } => Ok(Dice::Roll {
                count,
                sides: variables
                    .get(&sides)
                    .map(|var| *var)
                    .ok_or(ConvertDiceError::MissingVariable(sides))?
                    .floor() as u16,
            }),
            Self::VariableRoll { count, sides } => Ok(Dice::Roll {
                count: variables
                    .get(&count)
                    .map(|var| *var)
                    .ok_or(ConvertDiceError::MissingVariable(count))?
                    .floor() as u16,
                sides: variables
                    .get(&sides)
                    .map(|var| *var)
                    .ok_or(ConvertDiceError::MissingVariable(sides))?
                    .floor() as u16,
            }),
            Self::Sum(values) => {
                let mut dice_values = Vec::new();
                for value in values {
                    dice_values.push(value.to_dice(variables)?);
                }
                Ok(Dice::Sum(dice_values))
            }
            Self::Product(values) => {
                let mut dice_values = Vec::new();
                for value in values {
                    dice_values.push(value.to_dice(variables)?);
                }
                Ok(Dice::Product(dice_values))
            }
        }
    }
}

impl<T> Display for DiceFormula<T>
where
    T: Display + Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value}"),
            Self::Roll { count, sides } => write!(f, "{count}d{sides}"),
            Self::VariableRoll { count, sides } => write!(f, "{count}d{sides}"),
            Self::VariableSizeRoll { count, sides } => write!(f, "{count}d{sides}"),
            Self::VariableCountRoll { count, sides } => write!(f, "{count}d{sides}"),
            Self::Variable(variable) => write!(f, "{variable}"),
            Self::Sum(values) => {
                let mut iter = values.iter();

                if let Some(first) = iter.next() {
                    write!(f, "{first}")?;

                    for item in iter {
                        write!(f, " + {item}")?;
                    }
                }

                Ok(())
            }
            Self::Product(values) => {
                const fn requires_scope<T>(entry: &DiceFormula<T>) -> bool
                where
                    T: Ord,
                {
                    matches!(
                        entry,
                        DiceFormula::Roll { count: _, sides: _ }
                            | DiceFormula::Product(_)
                            | DiceFormula::Sum(_)
                    )
                }

                let mut iter = values.iter();

                if let Some(first) = iter.next() {
                    if requires_scope(first) {
                        write!(f, "[{first}]")?;
                    } else {
                        write!(f, "{first}")?;
                    }

                    for item in iter {
                        if requires_scope(item) {
                            write!(f, " * [{item}]")?;
                        } else {
                            write!(f, " * {item}")?;
                        }
                    }
                }

                Ok(())
            }
        }
    }
}

/// Errors returned when trying to convert from a [`DiceFormula`] to a [`Dice`]
#[derive(Debug)]
pub enum ConvertDiceError<T> {
    /// Indicates that a certain variable is missing from the provided variable set
    MissingVariable(T),
}

#[cfg(test)]
mod tests {
    use im::ordmap;

    use super::*;

    #[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
    pub enum TestVariables {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    }

    #[test]
    fn succeeds_when_fully_defined() {
        let values = vec![
            DiceFormula::Value(10f32),
            DiceFormula::Variable(TestVariables::A),
            DiceFormula::Roll {
                count: 10,
                sides: 5,
            },
            DiceFormula::VariableCountRoll {
                count: TestVariables::B,
                sides: 4,
            },
            DiceFormula::VariableSizeRoll {
                count: 4,
                sides: TestVariables::C,
            },
            DiceFormula::VariableRoll {
                count: TestVariables::D,
                sides: TestVariables::E,
            },
            DiceFormula::Sum(vec![
                DiceFormula::Value(10f32),
                DiceFormula::Variable(TestVariables::A),
                DiceFormula::Roll {
                    count: 10,
                    sides: 5,
                },
                DiceFormula::VariableCountRoll {
                    count: TestVariables::B,
                    sides: 4,
                },
                DiceFormula::VariableSizeRoll {
                    count: 4,
                    sides: TestVariables::C,
                },
                DiceFormula::VariableRoll {
                    count: TestVariables::D,
                    sides: TestVariables::E,
                },
            ]),
            DiceFormula::Product(vec![
                DiceFormula::Value(10f32),
                DiceFormula::Variable(TestVariables::A),
                DiceFormula::Roll {
                    count: 10,
                    sides: 5,
                },
                DiceFormula::VariableCountRoll {
                    count: TestVariables::B,
                    sides: 4,
                },
                DiceFormula::VariableSizeRoll {
                    count: 4,
                    sides: TestVariables::C,
                },
                DiceFormula::VariableRoll {
                    count: TestVariables::D,
                    sides: TestVariables::E,
                },
            ]),
        ];
        let map = ordmap! {
            TestVariables::A => 1f32,
            TestVariables::B => 2f32,
            TestVariables::C => 3f32,
            TestVariables::D => 4f32,
            TestVariables::E => 5f32
        };

        for item in values {
            let dice = item.to_dice(&map);
            assert!(dice.is_ok());
        }
    }

    #[test]
    fn fails_when_missing_variable() {
        let values = vec![
            DiceFormula::Variable(TestVariables::A),
            DiceFormula::VariableCountRoll {
                count: TestVariables::B,
                sides: 4,
            },
            DiceFormula::VariableSizeRoll {
                count: 4,
                sides: TestVariables::C,
            },
            DiceFormula::VariableRoll {
                count: TestVariables::D,
                sides: TestVariables::E,
            },
            DiceFormula::VariableRoll {
                count: TestVariables::F,
                sides: TestVariables::E,
            },
            DiceFormula::VariableRoll {
                count: TestVariables::E,
                sides: TestVariables::G,
            },
            DiceFormula::Sum(vec![
                DiceFormula::Variable(TestVariables::A),
                DiceFormula::VariableCountRoll {
                    count: TestVariables::B,
                    sides: 4,
                },
                DiceFormula::VariableSizeRoll {
                    count: 4,
                    sides: TestVariables::C,
                },
                DiceFormula::VariableRoll {
                    count: TestVariables::D,
                    sides: TestVariables::E,
                },
            ]),
            DiceFormula::Product(vec![
                DiceFormula::Variable(TestVariables::A),
                DiceFormula::VariableCountRoll {
                    count: TestVariables::B,
                    sides: 4,
                },
                DiceFormula::VariableSizeRoll {
                    count: 4,
                    sides: TestVariables::C,
                },
                DiceFormula::VariableRoll {
                    count: TestVariables::D,
                    sides: TestVariables::E,
                },
            ]),
        ];
        let map = ordmap! {
            TestVariables::F => 1f32,
            TestVariables::G => 2f32,
            TestVariables::H => 3f32,
            TestVariables::I => 4f32,
            TestVariables::J => 5f32
        };

        for item in values {
            let dice = item.to_dice(&map);
            assert!(dice.is_err());
        }
    }

    mod inserts_variables {
        use utils::float::ErrorMargin;

        use super::*;

        #[test]
        fn variable() {
            let formula = DiceFormula::Variable(TestVariables::A);
            let map = ordmap! {TestVariables::A => 2f32};
            let dice = formula.to_dice(&map).unwrap();
            assert!(match dice {
                Dice::Value(value) => value.within_margin(&2f32),
                _ => false,
            });
        }

        #[test]
        fn variable_count_roll() {
            let formula = DiceFormula::VariableCountRoll {
                count: TestVariables::B,
                sides: 12,
            };
            let map = ordmap! {TestVariables::B => 2f32};
            let dice = formula.to_dice(&map).unwrap();
            assert!(match dice {
                Dice::Roll { count: 2, sides: _ } => true,
                _ => false,
            });
        }

        #[test]
        fn variable_sides_roll() {
            let formula = DiceFormula::VariableSizeRoll {
                sides: TestVariables::C,
                count: 5,
            };
            let map = ordmap! {TestVariables::C => 2f32};
            let dice = formula.to_dice(&map).unwrap();
            assert!(match dice {
                Dice::Roll { count: _, sides: 2 } => true,
                _ => false,
            });
        }

        #[test]
        fn variable_roll() {
            let formula = DiceFormula::VariableRoll {
                sides: TestVariables::D,
                count: TestVariables::E,
            };
            let map = ordmap! {TestVariables::D => 2f32, TestVariables::E => 4f32};
            let dice = formula.to_dice(&map).unwrap();
            assert!(match dice {
                Dice::Roll { count: 4, sides: 2 } => true,
                _ => false,
            });
        }
    }
}
