use std::fmt::Display;

use rand::Rng;

/// Describes a dice roll
pub enum Dice {
    /// A hard-coded value
    Value(f32),
    /// The result of a roll
    Roll {
        /// The number of dice rolled
        count: u16,
        /// The number of sides on the dice
        sides: u16,
    },
    /// The sum of several sub-components
    Sum(Vec<Dice>),
    /// The product of several sub-components
    Product(Vec<Dice>),
}

impl Dice {
    /// Returns the minimum possible value that this expression can roll
    pub fn minimum(&self) -> f32 {
        match self {
            Self::Value(value) => *value,
            Self::Roll { count, sides: _ } => f32::from(*count),
            Self::Sum(values) => values.iter().map(Self::minimum).sum(),
            Self::Product(values) => values.iter().map(Self::minimum).product(),
        }
    }

    /// Returns the maximum possible value that this expression can roll
    pub fn maximum(&self) -> f32 {
        match self {
            Self::Value(value) => *value,
            Self::Roll { count, sides } => f32::from(count * sides),
            Self::Sum(values) => values.iter().map(Self::minimum).sum(),
            Self::Product(values) => values.iter().map(Self::minimum).product(),
        }
    }

    /// Rolls the dice to generate a random result.
    pub fn roll(&self) -> f32 {
        match self {
            Self::Value(value) => *value,
            Self::Roll { count, sides } => {
                let mut rng = rand::thread_rng();

                let mut total = 0;

                for _ in 0..*count {
                    total += rng.gen_range(1..=*sides);
                }

                f32::from(total)
            }
            Self::Sum(values) => values.iter().map(Self::roll).sum(),
            Self::Product(values) => values.iter().map(Self::roll).product(),
        }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value}"),
            Self::Roll { count, sides } => write!(f, "{count}d{sides}"),
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
                const fn requires_scope(entry: &Dice) -> bool {
                    matches!(
                        entry,
                        Dice::Roll { count: _, sides: _ } | Dice::Product(_) | Dice::Sum(_)
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

impl From<f32> for Dice {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::float::ErrorMargin;

    mod minimum {

        use super::*;

        #[test]
        fn value() {
            assert!(Dice::Value(10f32).minimum().within_margin(&10f32));
        }

        #[test]
        fn roll() {
            let dice = Dice::Roll {
                count: 5,
                sides: 10,
            };

            assert!(dice.minimum().within_margin(&5f32));
        }

        #[test]
        fn sum() {
            let dice = Dice::Sum(vec![Dice::Value(10f32), Dice::Value(2f32)]);

            assert!(dice.minimum().within_margin(&12f32));
        }

        #[test]
        fn product() {
            let dice = Dice::Product(vec![Dice::Value(2f32), Dice::Value(4f32)]);

            assert!(
                dice.minimum().within_margin(&8f32),
                "Dice should be 8f32, found {}",
                dice.minimum()
            );
        }
    }

    mod maximum {

        use super::*;

        #[test]
        fn value() {
            assert!(Dice::Value(10f32).maximum().within_margin(&10f32));
        }

        #[test]
        fn roll() {
            let dice = Dice::Roll {
                count: 5,
                sides: 10,
            };

            assert!(dice.maximum().within_margin(&50f32));
        }

        #[test]
        fn sum() {
            let dice = Dice::Sum(vec![Dice::Value(10f32), Dice::Value(2f32)]);

            assert!(dice.maximum().within_margin(&12f32));
        }

        #[test]
        fn product() {
            let dice = Dice::Product(vec![Dice::Value(2f32), Dice::Value(4f32)]);

            assert!(dice.maximum().within_margin(&8f32),);
        }
    }

    mod roll {
        use super::*;

        #[test]
        fn value() {
            let dice = Dice::Value(5f32);

            assert!(dice.roll().within_margin(&5f32));
        }

        #[test]
        fn roll() {
            let dice = Dice::Roll { count: 2, sides: 6 };

            for _ in 0..100 {
                let result = dice.roll();
                assert!(result >= 2f32);
                assert!(result <= 12f32);
            }
        }

        #[test]
        fn sum() {
            let dice = Dice::Sum(vec![
                Dice::Value(5f32),
                Dice::Value(5f32),
                Dice::Value(2f32),
            ]);

            assert!(dice.roll().within_margin(&12f32));
        }

        #[test]
        fn product() {
            let dice = Dice::Product(vec![Dice::Value(2f32), Dice::Value(3f32)]);

            assert!(dice.roll().within_margin(&6f32));
        }
    }
}
