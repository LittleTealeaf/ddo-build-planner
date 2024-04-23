use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition, Value},
    breakdowns::{Breakdowns, DiceStrategy},
    debug::DebugValue,
};
use core::{ops::Neg, str::FromStr};
use rust_decimal::Decimal;

/// Pushes a list of bonuses into a breakdown object and expects [`Attribute::Debug(0)`] to have
/// the specified value
fn expect_value<B, E>(bonuses: B, expected: E)
where
    B: IntoIterator<Item = Bonus>,
    E: Into<Decimal>,
{
    let mut breakdowns = Breakdowns::new();
    breakdowns.insert_bonuses(bonuses);
    let value = breakdowns.get_attribute(Attribute::Debug(0));
    let expected: Decimal = expected.into();
    assert_eq!(value, expected, "Expected {expected}, found {value}",);
}

mod value {

    use super::*;

    fn dbg_bonus(attribute: u8, value: Value) -> Bonus {
        Bonus::new(
            Attribute::Debug(attribute),
            BonusType::Stacking,
            value,
            None,
            BonusSource::Debug(0),
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
        expect_value([dbg_bonus(0, Value::try_from(10.0).unwrap().floor())], 10);
        expect_value([dbg_bonus(0, Value::try_from(10.01).unwrap().floor())], 10);
        expect_value([dbg_bonus(0, Value::try_from(10.99).unwrap().floor())], 10);
    }

    #[test]
    fn ceil() {
        expect_value([dbg_bonus(0, Value::try_from(10.5).unwrap().ceil())], 11);
        expect_value([dbg_bonus(0, Value::try_from(10.0).unwrap().ceil())], 10);
        expect_value([dbg_bonus(0, Value::try_from(10.01).unwrap().ceil())], 11);
        expect_value([dbg_bonus(0, Value::try_from(10.99).unwrap().ceil())], 11);
    }

    #[test]
    fn round() {
        expect_value([dbg_bonus(0, Value::try_from(10.5).unwrap().round())], 10);
        expect_value([dbg_bonus(0, Value::try_from(10.0).unwrap().round())], 10);
        expect_value([dbg_bonus(0, Value::try_from(10.01).unwrap().round())], 10);
        expect_value([dbg_bonus(0, Value::try_from(10.99).unwrap().round())], 11);
    }

    #[test]
    fn abs() {
        expect_value([dbg_bonus(0, Value::from(2).abs())], 2);
        expect_value([dbg_bonus(0, Value::from(-3).abs())], 3);
        expect_value([dbg_bonus(0, Value::from(0).abs())], 0);
    }

    #[test]
    fn condition() {
        expect_value([dbg_bonus(0, Value::condition(true, 10, 20))], 10);
        expect_value([dbg_bonus(0, Value::condition(false, 10, 20))], 20);
        expect_value(
            [dbg_bonus(
                0,
                Value::If {
                    condition: Condition::TRUE.into(),
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
                    condition: Condition::FALSE.into(),
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

    #[test]
    fn dice_average() {
        {
            let bonuses = [dbg_bonus(0, Value::dice(1, 6))];
            let expected = Decimal::from_str("3.5").unwrap();
            let mut breakdowns = Breakdowns::new();
            breakdowns.set_dice_strategy(DiceStrategy::Average);
            breakdowns.insert_bonuses(bonuses);
            let value = breakdowns.get_attribute(Attribute::Debug(0));
            assert_eq!(value, expected, "Expected {expected}, found {value}",);
        };
    }

    #[test]
    fn dice_minimum() {
        {
            let bonuses = [dbg_bonus(0, Value::dice(1, 6))];
            let expected = Decimal::from_str("1").unwrap();
            let mut breakdowns = Breakdowns::new();
            breakdowns.set_dice_strategy(DiceStrategy::Minimum);
            breakdowns.insert_bonuses(bonuses);
            let value = breakdowns.get_attribute(Attribute::Debug(0));
            assert_eq!(value, expected, "Expected {expected}, found {value}",);
        };
    }

    #[test]
    fn dice_maximum() {
        {
            let bonuses = [dbg_bonus(0, Value::dice(1, 6))];
            let expected = Decimal::from_str("6").unwrap();
            let mut breakdowns = Breakdowns::new();
            breakdowns.set_dice_strategy(DiceStrategy::Maximum);
            breakdowns.insert_bonuses(bonuses);
            let value = breakdowns.get_attribute(Attribute::Debug(0));
            assert_eq!(value, expected, "Expected {expected}, found {value}",);
        };
    }
}

mod condition {
    use super::*;

    #[allow(clippy::needless_pass_by_value)]
    fn test_condition(condition: Condition, expected: bool) {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonus(Bonus::new(
            DebugValue(0),
            DebugValue(0),
            10,
            condition.clone(),
            DebugValue(0),
        ));

        let value = breakdowns.get_attribute(Attribute::from(DebugValue(0)));
        let result = value == 10.into();

        assert_eq!(
            result, expected,
            "Found {result}, expected {expected}, for condition {condition}"
        );
    }

    #[test]
    fn not() {
        test_condition(Condition::Not(Box::new(Condition::TRUE)), false);
        test_condition(Condition::Not(Box::new(Condition::FALSE)), true);
    }

    #[test]
    fn greater_than() {
        test_condition(Condition::GreaterThan(10.into(), 5.into()), true);
        test_condition(Condition::GreaterThan(5.into(), 10.into()), false);
        test_condition(Condition::GreaterThan(10.into(), 10.into()), false);
    }

    #[test]
    fn less_than() {
        test_condition(Condition::LessThan(10.into(), 5.into()), false);
        test_condition(Condition::LessThan(5.into(), 10.into()), true);
        test_condition(Condition::LessThan(10.into(), 10.into()), false);
    }

    #[test]
    fn equal_to() {
        test_condition(Condition::EqualTo(10.into(), 5.into()), false);
        test_condition(Condition::EqualTo(5.into(), 10.into()), false);
        test_condition(Condition::EqualTo(10.into(), 10.into()), true);
    }

    #[test]
    fn and() {
        test_condition(Condition::FALSE & Condition::FALSE, false);
        test_condition(Condition::FALSE & Condition::TRUE, false);
        test_condition(Condition::TRUE & Condition::FALSE, false);
        test_condition(Condition::TRUE & Condition::TRUE, true);
    }

    #[test]
    fn or() {
        test_condition(Condition::FALSE | Condition::FALSE, false);
        test_condition(Condition::FALSE | Condition::TRUE, true);
        test_condition(Condition::TRUE | Condition::FALSE, true);
        test_condition(Condition::TRUE | Condition::TRUE, true);
    }

    #[test]
    fn xor() {
        test_condition(Condition::FALSE ^ Condition::FALSE, false);
        test_condition(Condition::FALSE ^ Condition::TRUE, true);
        test_condition(Condition::TRUE ^ Condition::FALSE, true);
        test_condition(Condition::TRUE ^ Condition::TRUE, false);
    }

    #[test]
    fn constant() {
        test_condition(Condition::Constant(false), false);
        test_condition(Condition::Constant(true), true);
    }
}

mod dynamic {
    use builder::{bonus::BonusTemplate, types::ability::Ability};

    use super::*;

    #[test]
    fn bonus_doesnt_apply_by_default() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.import_dynamic_bonus(
            Attribute::Debug(0),
            vec![BonusTemplate::new(DebugValue(1), DebugValue(1), 10, None)],
        );

        assert_eq!(breakdowns.get_attribute(DebugValue(1)), 0.into());
    }

    #[test]
    fn bonus_applies_if_value_exists() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.import_dynamic_bonus(
            Attribute::Debug(0),
            vec![BonusTemplate::new(DebugValue(1), DebugValue(1), 10, None)],
        );

        breakdowns.insert_bonus(Bonus::new(
            DebugValue(0),
            DebugValue(0),
            1,
            None,
            DebugValue(0),
        ));

        assert_eq!(breakdowns.get_attribute(DebugValue(1)), 10.into());
    }

    #[test]
    fn bonuses_get_cloned() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.import_dynamic_bonus(
            Attribute::Debug(0),
            vec![BonusTemplate::new(Ability::All, DebugValue(0), 10, None)],
        );

        let before = breakdowns.get_attribute(Ability::Constitution);

        breakdowns.insert_bonus(Bonus::new(
            Attribute::Debug(0),
            DebugValue(0),
            1,
            None,
            DebugValue(0),
        ));

        let after = breakdowns.get_attribute(Ability::Constitution);

        assert_eq!(after - before, 10.into());
    }
}

mod sources {
    use super::*;

    #[test]
    fn remove_source() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([
            Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(0),
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(0),
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(1),
            ),
        ]);
        breakdowns.remove_source(BonusSource::Debug(0));
        assert_eq!(breakdowns.get_attribute(Attribute::Debug(0)), 0.into());
        assert_eq!(breakdowns.get_attribute(Attribute::Debug(1)), 0.into());
        assert_eq!(breakdowns.get_attribute(Attribute::Debug(2)), 1.into());
    }

    #[test]
    fn remove_sources() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([
            Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(0),
            ),
            Bonus::new(
                Attribute::Debug(1),
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(1),
            ),
            Bonus::new(
                Attribute::Debug(2),
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(2),
            ),
        ]);
        breakdowns.remove_sources([BonusSource::Debug(0), BonusSource::Debug(1)]);
        assert_eq!(breakdowns.get_attribute(Attribute::Debug(0)), 0.into());
        assert_eq!(breakdowns.get_attribute(Attribute::Debug(1)), 0.into());
        assert_eq!(breakdowns.get_attribute(Attribute::Debug(2)), 1.into());
    }
}

mod stacking {
    use super::*;

    #[test]
    fn same_types_do_not_stack() {
        expect_value(
            [
                Bonus::new(DebugValue(0), DebugValue(0), 1, None, DebugValue(0)),
                Bonus::new(DebugValue(0), DebugValue(0), 2,None, DebugValue(0)),
            ],
            2,
        );
    }

    #[test]
    fn different_types_stack() {
        expect_value(
            [
                Bonus::new(DebugValue(0), DebugValue(0), 3, None, DebugValue(0)),
                Bonus::new(DebugValue(0), DebugValue(1), 2, None, DebugValue(0)),
            ],
            5,
        );
    }

    #[test]
    fn stacking_stacks_with_others() {
        expect_value(
            [
                Bonus::new(DebugValue(0), BonusType::Stacking, 1, None, DebugValue(0)),
                Bonus::new(DebugValue(0), DebugValue(0), 2, None, DebugValue(0)),
            ],
            3,
        );
    }

    #[test]
    fn stacking_stacks_with_stacking() {
        expect_value(
            [
                Bonus::new(DebugValue(0), BonusType::Stacking, 3, None, DebugValue(0)),
                Bonus::new(DebugValue(0), BonusType::Stacking, 5, None, DebugValue(0)),
            ],
            8,
        );
    }
}

mod breakdowns {
    use super::*;

    #[test]
    fn return_none_for_untracked_bonuses() {
        let mut breakdowns = Breakdowns::new();
        assert!(breakdowns.get_breakdowns(&Attribute::Debug(0)).is_none());
    }

    #[test]
    fn value_is_correct() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([
            Bonus::new(DebugValue(0), DebugValue(0), 6, None, DebugValue(0)),
            Bonus::new(DebugValue(0), DebugValue(1), 4, None, DebugValue(0)),
        ]);

        let expected = breakdowns.get_attribute(DebugValue(0));

        let breakdown = breakdowns
            .get_breakdowns(&Attribute::Debug(0))
            .expect("Expected Breakdowns");
        assert_eq!(breakdown.value(), &expected);
    }

    #[test]
    fn different_types_in_applied() {
        let a = Bonus::new(DebugValue(0), DebugValue(0), 6, None, DebugValue(0));
        let b = Bonus::new(DebugValue(0), DebugValue(1), 4, None, DebugValue(0));

        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([a.clone(), b.clone()]);
        let breakdown = breakdowns
            .get_breakdowns(&Attribute::Debug(0))
            .expect("Expected Breakdowns");

        assert!(breakdown.applied().iter().any(|i| i.bonus() == &a));
        assert!(breakdown.applied().iter().any(|i| i.bonus() == &b));
        assert!(!breakdown.overwritten().iter().any(|i| i.bonus() == &a));
        assert!(!breakdown.overwritten().iter().any(|i| i.bonus() == &b));
        assert!(!breakdown.disabled().iter().any(|i| i.bonus() == &a));
        assert!(!breakdown.disabled().iter().any(|i| i.bonus() == &b));
    }

    #[test]
    fn stacking_all_in_applied() {
        let a = Bonus::new(DebugValue(0), BonusType::Stacking, 6, None, DebugValue(0));
        let b = Bonus::new(DebugValue(0), BonusType::Stacking, 4, None, DebugValue(0));

        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([a.clone(), b.clone()]);
        let breakdown = breakdowns
            .get_breakdowns(&Attribute::Debug(0))
            .expect("Expected Breakdowns");

        assert!(breakdown.applied().iter().any(|i| i.bonus() == &a));
        assert!(breakdown.applied().iter().any(|i| i.bonus() == &b));
        assert!(!breakdown.overwritten().iter().any(|i| i.bonus() == &a));
        assert!(!breakdown.overwritten().iter().any(|i| i.bonus() == &b));
        assert!(!breakdown.disabled().iter().any(|i| i.bonus() == &a));
        assert!(!breakdown.disabled().iter().any(|i| i.bonus() == &b));
    }

    #[test]
    fn overwritten_separated() {
        let a = Bonus::new(DebugValue(0), DebugValue(0), 6, None, DebugValue(0));
        let b = Bonus::new(DebugValue(0), DebugValue(0), 4, None, DebugValue(0));

        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses([a.clone(), b.clone()]);
        let breakdown = breakdowns
            .get_breakdowns(&Attribute::Debug(0))
            .expect("Expected Breakdowns");

        assert!(breakdown.applied().iter().any(|i| i.bonus() == &a));
        assert!(!breakdown.applied().iter().any(|i| i.bonus() == &b));
        assert!(!breakdown.overwritten().iter().any(|i| i.bonus() == &a));
        assert!(breakdown.overwritten().iter().any(|i| i.bonus() == &b));
        assert!(!breakdown.disabled().iter().any(|i| i.bonus() == &a));
        assert!(!breakdown.disabled().iter().any(|i| i.bonus() == &b));
    }

    #[test]
    fn disabled_not_included() {
        let a = Bonus::new(
            DebugValue(0),
            DebugValue(0),
            6,
            Condition::from(false),
            DebugValue(0),
        );

        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonus(a.clone());
        let breakdown = breakdowns
            .get_breakdowns(&Attribute::Debug(0))
            .expect("Expected Breakdowns");

        assert!(!breakdown.applied().iter().any(|i| i.bonus() == &a));
        assert!(!breakdown.overwritten().iter().any(|i| i.bonus() == &a));
        assert!(breakdown.disabled().iter().any(|i| i.bonus() == &a));
    }
}

mod dice_strategy {

    use super::*;

    #[test]
    fn default_is_average() {
        assert_eq!(Breakdowns::new().dice_strategy(), DiceStrategy::Average);
    }

    #[test]
    fn setting_strategy_recalculates_bonuses() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonus(Bonus::new(
            DebugValue(0),
            DebugValue(0),
            Value::dice(1, 6),
            None,
            DebugValue(0),
        ));

        assert_eq!(
            breakdowns.get_attribute(DebugValue(0)),
            Decimal::from_str("3.5").unwrap()
        );

        breakdowns.set_dice_strategy(DiceStrategy::Minimum);
        assert_eq!(
            breakdowns.get_attribute(DebugValue(0)),
            Decimal::from_str("1").unwrap()
        );

        breakdowns.set_dice_strategy(DiceStrategy::Maximum);
        assert_eq!(
            breakdowns.get_attribute(DebugValue(0)),
            Decimal::from_str("6").unwrap()
        );
    }
}
