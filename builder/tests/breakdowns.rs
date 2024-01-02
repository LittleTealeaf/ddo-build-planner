mod calculate {
    use std::ops::Neg;

    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition, Value},
        breakdowns::Breakdowns,
    };
    use rust_decimal::Decimal;

    fn assert_value(bonuses: impl IntoIterator<Item = Bonus>, expected: impl Into<Decimal>) {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses(bonuses);
        let value = breakdowns.get_attribute(&Attribute::Debug(0));
        assert_eq!(value, expected.into());
    }

    fn dbg_bonus(attribute: u8, value: Value) -> Bonus {
        Bonus::new(
            Attribute::Debug(attribute),
            BonusType::Stacking,
            value,
            BonusSource::Debug(0),
            None,
        )
    }

    #[test]
    fn constant() {
        assert_value([dbg_bonus(0, Value::Const(10.into()))], 10);
    }

    #[test]
    fn attribute() {
        assert_value(
            [
                dbg_bonus(0, Value::Attribute(Attribute::Debug(1))),
                dbg_bonus(1, Value::Const(10.into())),
            ],
            10,
        );
    }

    #[test]
    fn add() {
        assert_value([dbg_bonus(0, Value::from(1) + Value::from(2))], 3);
    }

    #[test]
    fn sub() {
        assert_value([dbg_bonus(0, Value::from(5) - Value::from(2))], 3);
    }

    #[test]
    fn mul() {
        assert_value([dbg_bonus(0, Value::from(3) * Value::from(2))], 6);
    }

    #[test]
    fn div() {
        assert_value([dbg_bonus(0, Value::from(6) / Value::from(2))], 3);
    }

    #[test]
    fn rem() {
        assert_value([dbg_bonus(0, Value::from(5) % Value::from(2))], 1);
    }

    #[test]
    fn min() {
        assert_value([dbg_bonus(0, Value::from(5).min(Value::from(6)))], 5);
        assert_value([dbg_bonus(0, Value::from(6).min(Value::from(5)))], 5);
    }

    #[test]
    fn max() {
        assert_value([dbg_bonus(0, Value::from(5).max(Value::from(6)))], 6);
        assert_value([dbg_bonus(0, Value::from(6).max(Value::from(5)))], 6);
    }

    #[test]
    fn recip() {
        assert_value(
            [dbg_bonus(0, Value::from(2).recip())],
            Decimal::try_from(0.5).unwrap(),
        );
    }

    #[test]
    fn floor() {
        assert_value([dbg_bonus(0, Value::try_from(10.5).unwrap().floor())], 10);
    }

    #[test]
    fn ciel() {
        assert_value([dbg_bonus(0, Value::try_from(10.5).unwrap().ciel())], 11);
    }

    #[test]
    fn condition() {
        assert_value([dbg_bonus(0, Value::condition(true, 10, 20))], 10);
        assert_value([dbg_bonus(0, Value::condition(false, 10, 20))], 20);
        assert_value(
            [dbg_bonus(
                0,
                Value::If {
                    condition: Condition::Constant(true).into(),
                    if_true: Value::from(10).into(),
                    if_false: Value::from(20).into(),
                },
            )],
            10,
        );
        assert_value(
            [dbg_bonus(
                0,
                Value::If {
                    condition: Condition::Constant(false).into(),
                    if_true: Value::from(10).into(),
                    if_false: Value::from(20).into(),
                },
            )],
            20,
        );
    }

    #[test]
    fn negative() {
        assert_value([dbg_bonus(0, Value::from(1).neg())], -1);
    }

    #[test]
    fn average() {
        assert_value(
            [dbg_bonus(
                0,
                Value::mean([
                    Value::Const(1.into()),
                    Value::Const(2.into()),
                    Value::Const(3.into()),
                    Value::Const(4.into()),
                    Value::Const(5.into()),
                ]),
            )],
            3,
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
                1,
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1,
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1,
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
                1,
                BonusSource::Debug(0),
                None,
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1,
                BonusSource::Debug(1),
                None,
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1,
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

mod breakdowns {
    use builder::{attribute::Attribute, breakdowns::Breakdowns};

    #[test]
    fn return_none_for_untracked_bonuses() {
        let mut breakdowns = Breakdowns::new();
        assert!(breakdowns.get_breakdowns(&Attribute::Debug(0)).is_none());
    }
}
