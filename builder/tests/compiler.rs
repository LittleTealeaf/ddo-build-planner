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
            "Expected {expected}, found {value}",
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
    fn add() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(1f32) + Value::from(2f32),
                BonusSource::Debug(0),
                None,
            )],
            3f32,
        );
    }

    #[test]
    fn sub() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(5f32) - Value::from(2f32),
                BonusSource::Debug(0),
                None,
            )],
            3f32,
        );
    }

    #[test]
    fn mul() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(3f32) * Value::from(2f32),
                BonusSource::Debug(0),
                None,
            )],
            6f32,
        );
    }

    #[test]
    fn div() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(6f32) / Value::from(2f32),
                BonusSource::Debug(0),
                None,
            )],
            3f32,
        );
    }

    #[test]
    fn rem() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(5f32) % Value::from(2f32),
                BonusSource::Debug(0),
                None,
            )],
            1f32,
        );
    }

    #[test]
    fn min() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(5f32).min(Value::from(6f32)),
                BonusSource::Debug(0),
                None,
            )],
            5f32,
        );
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(6f32).min(Value::from(5f32)),
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
                Value::from(5f32).max(Value::from(6f32)),
                BonusSource::Debug(0),
                None,
            )],
            6f32,
        );
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(6f32).max(Value::from(5f32)),
                BonusSource::Debug(0),
                None,
            )],
            6f32,
        );
    }

    #[test]
    fn recip() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Box::new(Value::Value(2f32)).recip(),
                BonusSource::Debug(0),
                None,
            )],
            0.5f32,
        );
    }

    #[test]
    fn floor() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Box::new(Value::Value(10.5f32)).floor(),
                BonusSource::Debug(0),
                None,
            )],
            10f32,
        );
    }

    #[test]
    fn ciel() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(10.5f32).ciel(),
                BonusSource::Debug(0),
                None,
            )],
            11f32,
        );
    }

    #[test]
    fn condition() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::If {
                    condition: Box::new(Condition::Constant(true)),
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
                    condition: Box::new(Condition::Constant(false)),
                    if_true: Box::new(Value::Value(10f32)),
                    if_false: Box::new(Value::Value(20f32)),
                },
                BonusSource::Debug(0),
                None,
            )],
            20f32,
        );
    }

    #[test]
    fn negative() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                -Value::Value(1f32),
                BonusSource::Debug(0),
                None,
            )],
            -1f32,
        );
    }

    #[test]
    fn average() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::mean(vec![
                    Value::Value(1f32),
                    Value::Value(2f32),
                    Value::Value(3f32),
                    Value::Value(4f32),
                    Value::Value(5f32),
                ]),
                BonusSource::Debug(0),
                None,
            )],
            3f32,
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

    fn test_condition(condition: Condition, expected: bool, error: &'static str) {
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

        assert_eq!(
            result, expected,
            "Found {result}, expected {expected}, for condition {error}",
        );
    }

    #[test]
    fn not() {
        test_condition(
            Condition::Not(Box::new(Condition::Constant(true))),
            false,
            "!true",
        );
        test_condition(
            Condition::Not(Box::new(Condition::Constant(false))),
            true,
            "true",
        );
    }

    #[test]
    fn greater_than() {
        test_condition(
            Condition::GreaterThan(10f32.into(), 5f32.into()),
            true,
            "10 > 5",
        );
        test_condition(
            Condition::GreaterThan(5f32.into(), 10f32.into()),
            false,
            "5 > 10",
        );
        test_condition(
            Condition::GreaterThan(10f32.into(), 10f32.into()),
            false,
            "10 > 10",
        );
    }

    #[test]
    fn less_than() {
        test_condition(
            Condition::LessThan(10f32.into(), 5f32.into()),
            false,
            "10 < 5",
        );
        test_condition(
            Condition::LessThan(5f32.into(), 10f32.into()),
            true,
            "5 < 10",
        );
        test_condition(
            Condition::LessThan(10f32.into(), 10f32.into()),
            false,
            "10 < 10",
        );
    }

    #[test]
    fn equal_to() {
        test_condition(
            Condition::EqualTo(10f32.into(), 5f32.into()),
            false,
            "10 == 5",
        );
        test_condition(
            Condition::EqualTo(5f32.into(), 10f32.into()),
            false,
            "5 == 10",
        );
        test_condition(
            Condition::EqualTo(10f32.into(), 10f32.into()),
            true,
            "10 == 10",
        );
    }

    #[test]
    fn and() {
        test_condition(
            Condition::from(false) & Condition::from(false),
            false,
            "False and False",
        );
        test_condition(
            Condition::from(false) & Condition::from(true),
            false,
            "False and True",
        );
        test_condition(
            Condition::from(true) & Condition::from(false),
            false,
            "True and False",
        );
        test_condition(
            Condition::from(true) & Condition::from(true),
            true,
            "True and True",
        );
    }

    #[test]
    fn or() {
        test_condition(
            Condition::from(false) | Condition::from(false),
            false,
            "False and False",
        );
        test_condition(
            Condition::from(false) | Condition::from(true),
            true,
            "False and True",
        );
        test_condition(
            Condition::from(true) | Condition::from(false),
            true,
            "True and False",
        );
        test_condition(
            Condition::from(true) | Condition::from(true),
            true,
            "True and True",
        );
    }

    #[test]
    fn xor() {
        test_condition(
            Condition::from(false) ^ Condition::from(false),
            false,
            "False and False",
        );
        test_condition(
            Condition::from(false) ^ Condition::from(true),
            true,
            "False and True",
        );
        test_condition(
            Condition::from(true) ^ Condition::from(false),
            true,
            "True and False",
        );
        test_condition(
            Condition::from(true) ^ Condition::from(true),
            false,
            "True and True",
        );
    }

    #[test]
    fn const_true() {
        test_condition(Condition::Constant(true), true, "true");
    }

    #[test]
    fn const_false() {
        test_condition(Condition::Constant(false), false, "false");
    }
}

mod sources {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType},
        compiler::Compiler,
    };
    use utils::float::ErrorMargin;

    #[test]
    fn remove_source() {
        let mut compiler = Compiler::new();
        compiler.add_bonuses([
            Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                1f32.into(),
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1f32.into(),
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1f32.into(),
                BonusSource::Debug(1),
                None,
            ),
        ]);
        compiler.remove_source(BonusSource::Debug(0));
        assert!(compiler
            .get_attribute(&Attribute::Debug(0))
            .within_margin(&0f32));
        assert!(compiler
            .get_attribute(&Attribute::Debug(1))
            .within_margin(&0f32));
        assert!(compiler
            .get_attribute(&Attribute::Debug(2))
            .within_margin(&1f32));
    }

    #[test]
    fn remove_sources() {
        let mut compiler = Compiler::new();
        compiler.add_bonuses([
            Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                1f32.into(),
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1f32.into(),
                BonusSource::Debug(1),
                None,
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1f32.into(),
                BonusSource::Debug(2),
                None,
            ),
        ]);
        compiler.remove_sources([BonusSource::Debug(0), BonusSource::Debug(1)]);
        assert!(compiler
            .get_attribute(&Attribute::Debug(0))
            .within_margin(&0f32));
        assert!(compiler
            .get_attribute(&Attribute::Debug(1))
            .within_margin(&0f32));
        assert!(compiler
            .get_attribute(&Attribute::Debug(2))
            .within_margin(&1f32));
    }
}
