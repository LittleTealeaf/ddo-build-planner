mod calculate {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition, Value},
        compiler::Compiler,
    };
    use utils::float::ErrorMargin;

    fn test_bonuses<const L: usize>(bonuses: [Bonus; L], expected: f32) {
        let mut compiler = Compiler::default();
        compiler.add_bonuses(bonuses);
        let value = compiler.get_attribute(&Attribute::Debug(0));
        assert!(
            value.within_margin(&expected),
            "Expected {}, found {}",
            expected,
            value
        );
    }

    #[test]
    fn value() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::Value(10f32),
                BonusSource::Debug(0),
                None,
            )],
            10f32,
        );
    }

    #[test]
    fn attribute() {
        test_bonuses(
            [
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Stacking,
                    Value::Attribute(Attribute::Debug(1)),
                    BonusSource::Debug(0),
                    None,
                ),
                Bonus::new(
                    Attribute::Debug(1),
                    BonusType::Stacking,
                    Value::Value(10f32),
                    BonusSource::Debug(0),
                    None,
                ),
            ],
            10f32,
        );
    }

    #[test]
    fn sum() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::Sum(vec![Value::Value(6f32), Value::Value(5f32)]),
                BonusSource::Debug(0),
                None,
            )],
            11f32,
        );
    }

    #[test]
    fn product() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::Product(vec![Value::Value(6f32), Value::Value(5f32)]),
                BonusSource::Debug(0),
                None,
            )],
            30f32,
        );
    }

    #[test]
    fn min() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::Min(vec![Value::Value(6f32), Value::Value(5f32)]),
                BonusSource::Debug(0),
                None,
            )],
            5f32,
        );
    }

    #[test]
    fn max() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::Max(vec![Value::Value(6f32), Value::Value(5f32)]),
                BonusSource::Debug(0),
                None,
            )],
            6f32,
        );
    }

    #[test]
    fn floor() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::Floor(Box::new(Value::Value(10.5f32))),
                BonusSource::Debug(0),
                None,
            )],
            10f32,
        );
    }

    #[test]
    fn condition() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::If {
                    condition: Box::new(Condition::True),
                    if_true: Box::new(Value::Value(10f32)),
                    if_false: Box::new(Value::Value(20f32)),
                },
                BonusSource::Debug(0),
                None,
            )],
            10f32,
        );
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::If {
                    condition: Box::new(Condition::False),
                    if_true: Box::new(Value::Value(10f32)),
                    if_false: Box::new(Value::Value(20f32)),
                },
                BonusSource::Debug(0),
                None,
            )],
            20f32,
        );
    }
}

mod condition {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition, Value},
        compiler::Compiler,
    };
    use utils::float::ErrorMargin;

    fn test_condition(condition: Condition, expected: bool) {
        let mut compiler = Compiler::default();
        compiler.add_bonus(Bonus::new(
            Attribute::Debug(0),
            BonusType::Stacking,
            Value::Value(10f32),
            BonusSource::Base,
            Some(condition),
        ));
        let value = compiler.get_attribute(&Attribute::Debug(0));
        let result = value.within_margin(&10f32);

        assert_eq!(result, expected, "Found {}, expected {}", result, expected);
    }

    #[test]
    fn not() {
        test_condition(Condition::Not(Box::new(Condition::True)), false);
        test_condition(Condition::Not(Box::new(Condition::False)), true);
    }

    #[test]
    fn greater_than() {
        test_condition(Condition::GreaterThan(10f32.into(), 5f32.into()), true);
        test_condition(Condition::GreaterThan(5f32.into(), 10f32.into()), false);
        test_condition(Condition::GreaterThan(10f32.into(), 10f32.into()), false);
    }

    #[test]
    fn less_than() {
        test_condition(Condition::LessThan(10f32.into(), 5f32.into()), false);
        test_condition(Condition::LessThan(5f32.into(), 10f32.into()), true);
        test_condition(Condition::LessThan(10f32.into(), 10f32.into()), false);
    }

    #[test]
    fn equal_to() {
        test_condition(Condition::EqualTo(10f32.into(), 5f32.into()), false);
        test_condition(Condition::EqualTo(5f32.into(), 10f32.into()), false);
        test_condition(Condition::EqualTo(10f32.into(), 10f32.into()), true);
    }

    #[test]
    fn not_equal_to() {
        test_condition(Condition::NotEqualTo(10f32.into(), 5f32.into()), true);
        test_condition(Condition::NotEqualTo(5f32.into(), 10f32.into()), true);
        test_condition(Condition::NotEqualTo(10f32.into(), 10f32.into()), false);
    }

    #[test]
    fn any() {
        test_condition(
            Condition::Any(vec![Condition::True, Condition::False, Condition::False]),
            true,
        );
        test_condition(
            Condition::Any(vec![Condition::False, Condition::False, Condition::False]),
            false,
        );
        test_condition(
            Condition::Any(vec![Condition::True, Condition::True, Condition::True]),
            true,
        );
    }

    #[test]
    fn all() {
        test_condition(
            Condition::All(vec![Condition::True, Condition::False, Condition::False]),
            false,
        );
        test_condition(
            Condition::All(vec![Condition::False, Condition::False, Condition::False]),
            false,
        );
        test_condition(
            Condition::All(vec![Condition::True, Condition::True, Condition::True]),
            true,
        );
    }

    #[test]
    fn const_true() {
        test_condition(Condition::True, true);
    }

    #[test]
    fn const_false() {
        test_condition(Condition::False, false);
    }
}
