use builder::{attribute::Attribute, bonus::Bonus, breakdowns::Breakdowns};
use rust_decimal::Decimal;

/// Pushes a list of bonuses into a breakdown object and expects [`Attribute::Debug(0)`] to have
/// the specified value
fn expect_value(bonuses: impl IntoIterator<Item = Bonus>, expected: impl Into<Decimal>) {
    let mut breakdowns = Breakdowns::new();
    breakdowns.insert_bonuses(bonuses);
    let value = breakdowns.get_attribute(&Attribute::Debug(0));
    assert_eq!(value, expected.into());
}

mod value {
    use std::ops::Neg;

    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition, Value},
    };
    use rust_decimal::Decimal;

    use crate::expect_value;
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
        expect_value([dbg_bonus(0, Value::Const(10.into()))], 10);
    }

    #[test]
    fn attribute() {
        expect_value(
            [
                dbg_bonus(0, Value::Attribute(Attribute::Debug(1))),
                dbg_bonus(1, Value::Const(10.into())),
            ],
            10,
        );
    }

    #[test]
    fn add() {
        expect_value([dbg_bonus(0, Value::from(1) + Value::from(2))], 3);
    }

    #[test]
    fn sub() {
        expect_value([dbg_bonus(0, Value::from(5) - Value::from(2))], 3);
    }

    #[test]
    fn mul() {
        expect_value([dbg_bonus(0, Value::from(3) * Value::from(2))], 6);
    }

    #[test]
    fn div() {
        expect_value([dbg_bonus(0, Value::from(6) / Value::from(2))], 3);
    }

    #[test]
    fn rem() {
        expect_value([dbg_bonus(0, Value::from(5) % Value::from(2))], 1);
    }

    #[test]
    fn min() {
        expect_value([dbg_bonus(0, Value::from(5).min(Value::from(6)))], 5);
        expect_value([dbg_bonus(0, Value::from(6).min(Value::from(5)))], 5);
    }

    #[test]
    fn max() {
        expect_value([dbg_bonus(0, Value::from(5).max(Value::from(6)))], 6);
        expect_value([dbg_bonus(0, Value::from(6).max(Value::from(5)))], 6);
    }

    #[test]
    fn recip() {
        expect_value(
            [dbg_bonus(0, Value::from(2).recip())],
            Decimal::try_from(0.5).unwrap(),
        );
    }

    #[test]
    fn floor() {
        expect_value([dbg_bonus(0, Value::try_from(10.5).unwrap().floor())], 10);
    }

    #[test]
    fn ciel() {
        expect_value([dbg_bonus(0, Value::try_from(10.5).unwrap().ciel())], 11);
    }

    #[test]
    fn condition() {
        expect_value([dbg_bonus(0, Value::condition(true, 10, 20))], 10);
        expect_value([dbg_bonus(0, Value::condition(false, 10, 20))], 20);
        expect_value(
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
        expect_value(
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
        expect_value([dbg_bonus(0, Value::from(1).neg())], -1);
    }

    #[test]
    fn average() {
        expect_value(
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

mod stacking {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType},
    };

    use crate::expect_value;

    #[test]
    fn same_types_do_not_stack() {
        expect_value(
            [
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Debug(0),
                    1,
                    BonusSource::Debug(0),
                    None,
                ),
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Debug(0),
                    2,
                    BonusSource::Debug(0),
                    None,
                ),
            ],
            2,
        );
    }

    #[test]
    fn different_types_stack() {
        expect_value(
            [
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Debug(0),
                    3,
                    BonusSource::Debug(0),
                    None,
                ),
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Debug(1),
                    2,
                    BonusSource::Debug(0),
                    None,
                ),
            ],
            5,
        );
    }

    #[test]
    fn stacking_stacks_with_others() {
        expect_value(
            [
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Stacking,
                    1,
                    BonusSource::Debug(0),
                    None,
                ),
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Debug(1),
                    2,
                    BonusSource::Debug(0),
                    None,
                ),
            ],
            3,
        );
    }

    #[test]
    fn stacking_stacks_with_stacking() {
        expect_value(
            [
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Stacking,
                    3,
                    BonusSource::Debug(0),
                    None,
                ),
                Bonus::new(
                    Attribute::Debug(0),
                    BonusType::Stacking,
                    5,
                    BonusSource::Debug(0),
                    None,
                ),
            ],
            8,
        );
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
