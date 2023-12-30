mod calculate {
    use std::{ops::Neg, str::FromStr};

    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition, Value},
        breakdowns::Breakdowns,
    };
    use rust_decimal::Decimal;

    fn test_bonuses(bonuses: impl IntoIterator<Item = Bonus>, expected: Decimal) {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses(bonuses);
        let value = breakdowns.get_attribute(&Attribute::Debug(0));
        assert_eq!(value, expected);
        // assert!(
        //     value.within_margin(&expected),
        //     "Expected {expected}, found {value}",
        // );
    }

    #[test]
    fn value() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::Const(10.into()),
                BonusSource::Debug(0),
                None,
            )],
            10.into(),
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
                    Value::Const(10.into()),
                    BonusSource::Debug(0),
                    None,
                ),
            ],
            10.into(),
        );
    }

    #[test]
    fn add() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(1) + Value::from(2),
                BonusSource::Debug(0),
                None,
            )],
            3.into(),
        );
    }

    #[test]
    fn sub() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(5) - Value::from(2),
                BonusSource::Debug(0),
                None,
            )],
            3.into(),
        );
    }

    #[test]
    fn mul() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(3) * Value::from(2),
                BonusSource::Debug(0),
                None,
            )],
            6.into(),
        );
    }

    #[test]
    fn div() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(6) / Value::from(2),
                BonusSource::Debug(0),
                None,
            )],
            3.into(),
        );
    }

    #[test]
    fn rem() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(5) % Value::from(2),
                BonusSource::Debug(0),
                None,
            )],
            1.into(),
        );
    }

    #[test]
    fn min() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(5).min(Value::from(6)),
                BonusSource::Debug(0),
                None,
            )],
            5.into(),
        );
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(6).min(Value::from(5)),
                BonusSource::Debug(0),
                None,
            )],
            5.into(),
        );
    }

    #[test]
    fn max() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(5).max(Value::from(6)),
                BonusSource::Debug(0),
                None,
            )],
            6.into(),
        );
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(6).max(Value::from(5)),
                BonusSource::Debug(0),
                None,
            )],
            6.into(),
        );
    }

    #[test]
    fn recip() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Box::new(Value::Const(2.into())).recip(),
                BonusSource::Debug(0),
                None,
            )],
            Decimal::ONE / Decimal::TWO,
        );
    }

    #[test]
    fn floor() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Box::new(Value::Const(Decimal::from_f32_retain(10.5).unwrap())).floor(),
                BonusSource::Debug(0),
                None,
            )],
            10.into(),
        );
    }

    #[test]
    fn ciel() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::from(Decimal::from_str("10.5").unwrap()).ciel(),
                BonusSource::Debug(0),
                None,
            )],
            11.into(),
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
                    if_true: Box::new(Value::Const(10.into())),
                    if_false: Box::new(Value::Const(20.into())),
                },
                BonusSource::Debug(0),
                None,
            )],
            10.into(),
        );
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::If {
                    condition: Box::new(Condition::Constant(false)),
                    if_true: Box::new(Value::Const(10.into())),
                    if_false: Box::new(Value::Const(20.into())),
                },
                BonusSource::Debug(0),
                None,
            )],
            20.into(),
        );
    }

    #[test]
    fn negative() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                -Value::Const(1.into()),
                BonusSource::Debug(0),
                None,
            )],
            Decimal::ONE.neg(),
        );
    }

    #[test]
    fn average() {
        test_bonuses(
            [Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                Value::mean([
                    Value::Const(1.into()),
                    Value::Const(2.into()),
                    Value::Const(3.into()),
                    Value::Const(4.into()),
                    Value::Const(5.into()),
                ]),
                BonusSource::Debug(0),
                None,
            )],
            3.into(),
        );
    }
}

mod condition {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition, Value},
        breakdowns::Breakdowns,
    };

    fn test_condition(condition: Condition, expected: bool, error: &'static str) {
        let mut breakdowns = Breakdowns::default();
        breakdowns.insert_bonus(Bonus::new(
            Attribute::Debug(0),
            BonusType::Stacking,
            Value::Const(10.into()),
            BonusSource::Base,
            Some(condition),
        ));
        let value = breakdowns.get_attribute(&Attribute::Debug(0));
        let result = value == 10.into();

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
        test_condition(Condition::GreaterThan(10.into(), 5.into()), true, "10 > 5");
        test_condition(Condition::GreaterThan(5.into(), 10.into()), false, "5 > 10");
        test_condition(
            Condition::GreaterThan(10.into(), 10.into()),
            false,
            "10 > 10",
        );
    }

    #[test]
    fn less_than() {
        test_condition(Condition::LessThan(10.into(), 5.into()), false, "10 < 5");
        test_condition(Condition::LessThan(5.into(), 10.into()), true, "5 < 10");
        test_condition(Condition::LessThan(10.into(), 10.into()), false, "10 < 10");
    }

    #[test]
    fn equal_to() {
        test_condition(Condition::EqualTo(10.into(), 5.into()), false, "10 == 5");
        test_condition(Condition::EqualTo(5.into(), 10.into()), false, "5 == 10");
        test_condition(Condition::EqualTo(10.into(), 10.into()), true, "10 == 10");
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
        breakdowns::Breakdowns,
    };

    #[test]
    fn remove_source() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([
            Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                1.into(),
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1.into(),
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1.into(),
                BonusSource::Debug(1),
                None,
            ),
        ]);
        breakdowns.remove_source(BonusSource::Debug(0));
        assert!(breakdowns.get_attribute(&Attribute::Debug(0)) == 0.into());
        assert!(breakdowns.get_attribute(&Attribute::Debug(1)) == 0.into());
        assert!(breakdowns.get_attribute(&Attribute::Debug(2)) == 1.into());
    }

    #[test]
    fn remove_sources() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([
            Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                1.into(),
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1.into(),
                BonusSource::Debug(1),
                None,
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1.into(),
                BonusSource::Debug(2),
                None,
            ),
        ]);
        breakdowns.remove_sources([BonusSource::Debug(0), BonusSource::Debug(1)]);
        assert!(breakdowns.get_attribute(&Attribute::Debug(0)) == 0.into());
        assert!(breakdowns.get_attribute(&Attribute::Debug(1)) == 0.into());
        assert!(breakdowns.get_attribute(&Attribute::Debug(2)) == 1.into());
    }
}
