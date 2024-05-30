use builder::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusType, Condition, Depth, HasDice, ToValue, Value},
};
use core::ops::{Add, Div, Mul, Not, Rem, Sub};

mod has_dependency {

    use builder::debug::DebugValue;

    use super::*;

    #[test]
    fn value_from_bonus() {
        let bonus = Bonus::new(
            DebugValue(0),
            BonusType::Stacking,
            Attribute::Debug(1),
            DebugValue(0),
        );

        assert!(bonus.has_attr_dependency(&Attribute::Debug(1)));
        assert!(!bonus.has_attr_dependency(&Attribute::Debug(2)));
    }

    #[test]
    fn conditional_from_bonus() {
        let bonus = Bonus::new(
            DebugValue(0),
            BonusType::Stacking,
            Value::Const(10.into()),
            DebugValue(0),
        )
        .with_condition(Condition::has(DebugValue(1)));

        assert!(bonus.has_attr_dependency(&Attribute::Debug(1)));
        assert!(!bonus.has_attr_dependency(&Attribute::Debug(2)));
    }

    mod value {
        use super::*;

        #[test]
        fn value() {
            let value = Value::Const(10.into());

            assert!(!value.has_attr_dependency(&Attribute::Debug(0)));
        }

        #[test]
        fn dice() {
            let value = Value::dice(Attribute::Debug(0), Attribute::Debug(1));
            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn attribute() {
            let value = Value::Attribute(Attribute::Debug(0));

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(1)));
        }

        #[test]
        fn add() {
            let value = Value::Add(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn sub() {
            let value = Value::Sub(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn mul() {
            let value = Value::Mul(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn div() {
            let value = Value::Div(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn rem() {
            let value = Value::Rem(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn min() {
            let value = Value::Min(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(10).into(),
            );

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(1)));
        }

        #[test]
        fn max() {
            let value = Value::Max(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn floor() {
            let value = Value::Floor(Value::Attribute(Attribute::Debug(0)).into());

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(1)));
        }

        #[test]
        fn if_value() {
            let value = Value::If {
                condition: Condition::GreaterThan(Attribute::Debug(0).into(), 1.into()).into(),
                if_true: <Box<Value>>::from(Value::from(Attribute::Debug(1))),
                if_false: Value::from(Attribute::Debug(2)).into(),
            };

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(value.has_attr_dependency(&Attribute::Debug(1)));
            assert!(value.has_attr_dependency(&Attribute::Debug(2)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(3)));
        }

        #[test]
        fn abs() {
            let value = Value::from(Attribute::Debug(0)).abs();

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(1)));
        }

        #[test]
        fn ceil() {
            let value = Value::from(Attribute::Debug(0)).ceil();

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(1)));
        }

        #[test]
        fn round() {
            let value = Value::from(Attribute::Debug(0)).round();

            assert!(value.has_attr_dependency(&Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(&Attribute::Debug(1)));
        }
    }

    mod condition {
        use core::ops::Not;

        use builder::{
            attribute::{Attribute, AttributeDependencies},
            bonus::{Condition, Value},
        };

        fn attr_condition(n: usize) -> Condition {
            Value::from(Attribute::Debug(n)).equal_to(0.into())
        }

        #[test]
        fn not() {
            let condition = attr_condition(0).not();
            assert!(condition.has_attr_dependency(&Attribute::Debug(0)));
            assert!(!condition.has_attr_dependency(&Attribute::Debug(1)));
        }

        #[test]
        fn greater_than() {
            let condition =
                Value::from(Attribute::Debug(0)).greater_than(Attribute::Debug(1).into());
            assert!(condition.has_attr_dependency(&Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn less_than() {
            let condition = Value::from(Attribute::Debug(0)).less_than(Attribute::Debug(1).into());
            assert!(condition.has_attr_dependency(&Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn equal_to() {
            let condition = Value::from(Attribute::Debug(0)).equal_to(Attribute::Debug(1).into());
            assert!(condition.has_attr_dependency(&Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn constant() {
            let condition = Condition::from(false);
            // To make sure it's not just returning 0
            assert!(!condition.has_attr_dependency(&Attribute::Debug(0)));
        }

        #[test]
        fn and() {
            let condition = attr_condition(0) & attr_condition(1);
            assert!(condition.has_attr_dependency(&Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn or() {
            let condition = attr_condition(0) | attr_condition(1);
            assert!(condition.has_attr_dependency(&Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(&Attribute::Debug(2)));
        }

        #[test]
        fn xor() {
            let condition = attr_condition(0) ^ attr_condition(1);
            assert!(condition.has_attr_dependency(&Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(&Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(&Attribute::Debug(2)));
        }
    }
}

mod include_dependencies {
    use builder::debug::DebugValue;

    use super::*;

    #[test]
    fn conditional_from_bonus() {
        let bonus = Bonus::new(
            DebugValue(0),
            BonusType::Stacking,
            Attribute::Debug(1),
            DebugValue(0),
        );

        let dependencies = bonus.get_attr_dependencies();
        assert!(dependencies.contains(&Attribute::Debug(1)));
        assert!(!dependencies.contains(&Attribute::Debug(2)));
    }

    #[test]
    fn value_from_bonus() {
        let bonus = Bonus::new(
            DebugValue(0),
            BonusType::Stacking,
            Attribute::Debug(1),
            DebugValue(0),
        );

        let dependencies = bonus.get_attr_dependencies();
        assert!(dependencies.contains(&Attribute::Debug(1)));
        assert!(!dependencies.contains(&Attribute::Debug(2)));
    }

    mod value {
        use super::*;

        #[test]
        fn value() {
            let value = Value::Const(10.into());
            let deps = value.get_attr_dependencies();

            assert!(!deps.contains(&Attribute::Debug(0)));
        }

        #[test]
        fn dice() {
            let value = Value::dice(Attribute::Debug(0), Attribute::Debug(1));
            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn attribute() {
            let value = Value::Attribute(Attribute::Debug(0));

            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(!deps.contains(&Attribute::Debug(1)));
        }

        #[test]
        fn add() {
            let value = Value::Add(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn sub() {
            let value = Value::Sub(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn mul() {
            let value = Value::Mul(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn div() {
            let value = Value::Div(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn rem() {
            let value = Value::Rem(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn min() {
            let value = Value::Min(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(10).into(),
            );

            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(!deps.contains(&Attribute::Debug(1)));
        }

        #[test]
        fn max() {
            let value = Value::Max(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );

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
            let value = Value::If {
                condition: Condition::GreaterThan(Attribute::Debug(0).into(), 1.into()).into(),
                if_true: Value::from(Attribute::Debug(1)).into(),
                if_false: Value::from(Attribute::Debug(2)).into(),
            };

            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(deps.contains(&Attribute::Debug(2)));
            assert!(!deps.contains(&Attribute::Debug(3)));
        }

        #[test]
        fn abs() {
            let value = Value::from(Attribute::Debug(0)).abs();
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(!deps.contains(&Attribute::Debug(1)));
        }

        #[test]
        fn ceil() {
            let value = Value::from(Attribute::Debug(0)).ceil();
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(!deps.contains(&Attribute::Debug(1)));
        }

        #[test]
        fn round() {
            let value = Value::from(Attribute::Debug(0)).round();
            let deps = value.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(!deps.contains(&Attribute::Debug(1)));
        }
    }

    mod condition {
        use core::ops::Not;

        use builder::{
            attribute::{Attribute, AttributeDependencies},
            bonus::{Condition, Value},
        };

        fn attr_condition(n: usize) -> Condition {
            Value::from(Attribute::Debug(n)).equal_to(0.into())
        }

        #[test]
        fn not() {
            let value = attr_condition(0).not();
            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(!deps.contains(&Attribute::Debug(1)));
        }

        #[test]
        fn greater_than() {
            let value = Value::from(Attribute::Debug(0)).greater_than(Attribute::Debug(1).into());
            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn less_than() {
            let value = Value::from(Attribute::Debug(0)).less_than(Attribute::Debug(1).into());
            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn equal_to() {
            let value = Value::from(Attribute::Debug(0)).equal_to(Attribute::Debug(1).into());
            let deps = value.get_attr_dependencies();

            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn constant() {
            let value = Condition::from(false);
            let deps = value.get_attr_dependencies();
            assert!(deps.is_empty());
        }

        #[test]
        fn and() {
            let condition = attr_condition(0) & attr_condition(1);
            let deps = condition.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn or() {
            let condition = attr_condition(0) | attr_condition(1);
            let deps = condition.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }

        #[test]
        fn xor() {
            let condition = attr_condition(0) ^ attr_condition(1);
            let deps = condition.get_attr_dependencies();
            assert!(deps.contains(&Attribute::Debug(0)));
            assert!(deps.contains(&Attribute::Debug(1)));
            assert!(!deps.contains(&Attribute::Debug(2)));
        }
    }
}

mod has_dice {
    use super::*;

    mod value {

        use super::*;

        fn dice() -> Value {
            Value::dice(1, 6)
        }

        #[test]
        fn constant() {
            assert!(!Value::from(0).has_dice());
        }

        #[test]
        fn attribute() {
            assert!(!Value::Attribute(Attribute::Debug(0)).has_dice());
        }

        #[test]
        fn min() {
            assert!(!Value::min(Value::from(0), Value::from(0)).has_dice());
            assert!(Value::min(dice(), Value::from(0)).has_dice());
            assert!(Value::min(Value::from(0), dice()).has_dice());
        }

        #[test]
        fn max() {
            assert!(!Value::max(Value::from(0), Value::from(0)).has_dice());
            assert!(Value::max(dice(), Value::from(0)).has_dice());
            assert!(Value::max(Value::from(0), dice()).has_dice());
        }

        #[test]
        fn add() {
            assert!(!(0.to_value() + 0.to_value()).has_dice());
            assert!((dice() + 0.to_value()).has_dice());
            assert!((0.to_value() + dice()).has_dice());
        }

        #[test]
        fn sub() {
            assert!(!(0.to_value() - 0.to_value()).has_dice());
            assert!((dice() - 0.to_value()).has_dice());
            assert!((0.to_value() - dice()).has_dice());
        }

        #[test]
        fn mul() {
            assert!(!(0.to_value() * 0.to_value()).has_dice());
            assert!((dice() * 0.to_value()).has_dice());
            assert!((0.to_value() * dice()).has_dice());
        }

        #[test]
        fn div() {
            assert!(!(0.to_value() / 0.to_value()).has_dice());
            assert!((dice() / 0.to_value()).has_dice());
            assert!((0.to_value() / dice()).has_dice());
        }

        #[test]
        fn rem() {
            assert!(!(0.to_value() % 0.to_value()).has_dice());
            assert!((dice() % 0.to_value()).has_dice());
            assert!((0.to_value() % dice()).has_dice());
        }

        #[test]
        fn abs() {
            assert!(!0.to_value().abs().has_dice());
            assert!(dice().abs().has_dice());
        }

        #[test]
        fn round() {
            assert!(!0.to_value().round().has_dice());
            assert!(dice().round().has_dice());
        }

        #[test]
        fn ceil() {
            assert!(!0.to_value().ceil().has_dice());
            assert!(dice().ceil().has_dice());
        }

        #[test]
        fn floor() {
            assert!(!0.to_value().floor().has_dice());
            assert!(dice().floor().has_dice());
        }

        #[test]
        fn condition() {
            assert!(!Value::condition(false, 0, 0).has_dice());
            assert!(Value::condition(dice().greater_than(0.to_value()), 0, 0).has_dice());
            assert!(Value::condition(false, dice(), 0).has_dice());
            assert!(Value::condition(false, 0, dice()).has_dice());
        }
    }

    mod condition {

        use super::*;

        fn dice() -> Value {
            Value::dice(1, 6)
        }

        fn dice_cond() -> Condition {
            dice().greater_than(0.to_value())
        }

        fn no_cond() -> Condition {
            0.to_value().greater_than(0.to_value())
        }

        #[test]
        fn not() {
            assert!(!no_cond().not().has_dice());
            assert!(dice_cond().not().has_dice());
        }

        #[test]
        fn less_than() {
            assert!(!0.to_value().less_than(0.to_value()).has_dice());
            assert!(dice().less_than(0.to_value()).has_dice());
            assert!(0.to_value().less_than(dice()).has_dice());
        }

        #[test]
        fn equal_to() {
            assert!(!0.to_value().equal_to(0.to_value()).has_dice());
            assert!(dice().equal_to(0.to_value()).has_dice());
            assert!(0.to_value().equal_to(dice()).has_dice());
        }

        #[test]
        fn constant() {
            assert!(!Condition::Constant(false).has_dice());
            assert!(!Condition::Constant(true).has_dice());
        }

        #[test]
        fn and() {
            assert!(!(no_cond() & no_cond()).has_dice());
            assert!((dice_cond() & no_cond()).has_dice());
            assert!((no_cond() & dice_cond()).has_dice());
        }

        #[test]
        fn or() {
            assert!(!(no_cond() | no_cond()).has_dice());
            assert!((dice_cond() | no_cond()).has_dice());
            assert!((no_cond() | dice_cond()).has_dice());
        }

        #[test]
        fn xor() {
            assert!(!(no_cond() ^ no_cond()).has_dice());
            assert!((dice_cond() ^ no_cond()).has_dice());
            assert!((no_cond() ^ dice_cond()).has_dice());
        }
    }
}

mod depth {
    use super::*;

    fn value_depth(depth: usize) -> Value {
        assert_ne!(depth, 0, "Depth cannot be 0");
        let mut value = 0.to_value();

        for _ in 1..depth {
            value = value.abs();
        }

        value
    }

    #[test]
    fn test_value_depth() {
        assert_eq!(value_depth(5).get_depth(), 5);
        assert_eq!(value_depth(6).get_depth(), 6);
    }

    fn condition_depth(depth: usize) -> Condition {
        assert_ne!(depth, 0, "Depth cannot be 0");
        let mut condition = Condition::from(true);

        for _ in 1..depth {
            condition = condition.not();
        }

        condition
    }

    #[test]
    fn test_condition_depth() {
        assert_eq!(condition_depth(5).get_depth(), 5);
        assert_eq!(condition_depth(6).get_depth(), 6);
    }

    mod value {

        use super::*;

        #[test]
        fn constant() {
            assert_eq!(Value::from(3).get_depth(), 1);
        }

        #[test]
        fn attribute() {
            assert_eq!(Attribute::Debug(0).to_value().get_depth(), 1);
        }

        #[test]
        fn min() {
            assert_eq!(Value::min(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::min(value_depth(5), value_depth(3)).get_depth(), 6);
        }
        #[test]
        fn max() {
            assert_eq!(Value::max(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::max(value_depth(5), value_depth(3)).get_depth(), 6);
        }
        #[test]
        fn add() {
            assert_eq!(Value::add(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::add(value_depth(5), value_depth(3)).get_depth(), 6);
        }
        #[test]
        fn sub() {
            assert_eq!(Value::sub(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::sub(value_depth(5), value_depth(3)).get_depth(), 6);
        }

        #[test]
        fn mul() {
            assert_eq!(Value::mul(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::mul(value_depth(5), value_depth(3)).get_depth(), 6);
        }

        #[test]
        fn div() {
            assert_eq!(Value::div(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::div(value_depth(5), value_depth(3)).get_depth(), 6);
        }

        #[test]
        fn rem() {
            assert_eq!(Value::rem(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::rem(value_depth(5), value_depth(3)).get_depth(), 6);
        }

        #[test]
        fn round() {
            assert_eq!(Value::round(value_depth(3)).get_depth(), 4);
        }

        #[test]
        fn abs() {
            assert_eq!(Value::abs(value_depth(3)).get_depth(), 4);
        }

        #[test]
        fn floor() {
            assert_eq!(Value::floor(value_depth(3)).get_depth(), 4);
        }

        #[test]
        fn dice() {
            assert_eq!(Value::dice(value_depth(2), value_depth(3)).get_depth(), 4);
            assert_eq!(Value::dice(value_depth(5), value_depth(3)).get_depth(), 6);
        }
    }

    mod condition {
        use super::*;

        #[test]
        fn not() {
            assert_eq!(condition_depth(1).not().get_depth(), 2);
        }

        #[test]
        fn greater_than() {
            assert_eq!(value_depth(1).greater_than(value_depth(2)).get_depth(), 3);
            assert_eq!(value_depth(2).greater_than(value_depth(1)).get_depth(), 3);
        }

        #[test]
        fn less_than() {
            assert_eq!(value_depth(1).less_than(value_depth(2)).get_depth(), 3);
            assert_eq!(value_depth(2).less_than(value_depth(1)).get_depth(), 3);
        }

        #[test]
        fn equal_to() {
            assert_eq!(value_depth(1).equal_to(value_depth(2)).get_depth(), 3);
            assert_eq!(value_depth(2).equal_to(value_depth(1)).get_depth(), 3);
        }

        #[test]
        fn constant() {
            assert_eq!(Condition::Constant(true).get_depth(), 1);
            assert_eq!(Condition::Constant(false).get_depth(), 1);
        }

        #[test]
        fn and() {
            assert_eq!(condition_depth(2).and(condition_depth(1)).get_depth(), 3);
            assert_eq!(condition_depth(1).and(condition_depth(2)).get_depth(), 3);
        }

        #[test]
        fn or() {
            assert_eq!(condition_depth(2).or(condition_depth(1)).get_depth(), 3);
            assert_eq!(condition_depth(1).or(condition_depth(2)).get_depth(), 3);
        }

        #[test]
        fn xor() {
            assert_eq!(condition_depth(2).xor(condition_depth(1)).get_depth(), 3);
            assert_eq!(condition_depth(1).xor(condition_depth(2)).get_depth(), 3);
        }
    }
}
