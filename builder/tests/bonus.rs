use builder::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, BonusType, Condition, Value},
};

mod has_dependency {

    use builder::debug::DebugValue;

    use super::*;

    #[test]
    /// Tests that querying a bonus dependency will include the value dependencies
    fn gets_value_dependency() {
        let bonus = Bonus::new(
            DebugValue(0),
            BonusType::Stacking,
            Attribute::Debug(1),
            DebugValue(0),
            None,
        );

        assert!(bonus.has_attr_dependency(Attribute::Debug(1)));
        assert!(!bonus.has_attr_dependency(Attribute::Debug(2)));
    }

    #[test]
    /// Tests that querying a bonus dependency will include the conditional dependencies
    fn gets_conditional_dependency() {
        let bonus = Bonus::new(
            DebugValue(0),
            BonusType::Stacking,
            Value::Const(10.into()),
            DebugValue(0),
            Some(Condition::has(DebugValue(1))),
        );

        assert!(bonus.has_attr_dependency(Attribute::Debug(1)));
        assert!(!bonus.has_attr_dependency(Attribute::Debug(2)));
    }

    mod value {
        use super::*;

        #[test]
        fn value() {
            let value = Value::Const(10.into());

            assert!(!value.has_attr_dependency(Attribute::Debug(0)));
        }

        #[test]
        fn attribute() {
            let value = Value::Attribute(Attribute::Debug(0));

            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(Attribute::Debug(1)));
        }

        #[test]
        fn add() {
            let value = Value::Add(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(value.has_attr_dependency(Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn sub() {
            let value = Value::Sub(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(value.has_attr_dependency(Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn mul() {
            let value = Value::Mul(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(value.has_attr_dependency(Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn div() {
            let value = Value::Div(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(value.has_attr_dependency(Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn rem() {
            let value = Value::Rem(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );
            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(value.has_attr_dependency(Attribute::Debug(1)));
            assert!(!value.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn min() {
            let value = Value::Min(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(10).into(),
            );

            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(!value.has_attr_dependency(Attribute::Debug(1)));
        }

        #[test]
        fn max() {
            let value = Value::Max(
                Value::from(Attribute::Debug(0)).into(),
                Value::from(Attribute::Debug(1)).into(),
            );

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
            let value = Value::If {
                condition: Condition::GreaterThan(Attribute::Debug(0).into(), 1.into()).into(),
                if_true: <Box<Value>>::from(Value::from(Attribute::Debug(1))),
                if_false: Value::from(Attribute::Debug(2)).into(),
            };

            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(value.has_attr_dependency(Attribute::Debug(1)));
            assert!(value.has_attr_dependency(Attribute::Debug(2)));
            assert!(!value.has_attr_dependency(Attribute::Debug(3)));
        }
    }

    mod condition {
        use std::ops::Not;

        use builder::{
            attribute::{Attribute, AttributeDependencies},
            bonus::{Condition, Value},
        };

        fn attr_condition(n: u8) -> Condition {
            Value::from(Attribute::Debug(n)).equal_to(0.into())
        }

        #[test]
        fn not() {
            let condition = attr_condition(0).not();
            assert!(condition.has_attr_dependency(Attribute::Debug(0)));
            assert!(!condition.has_attr_dependency(Attribute::Debug(1)));
        }

        #[test]
        fn greater_than() {
            let condition =
                Value::from(Attribute::Debug(0)).greater_than(Attribute::Debug(1).into());
            assert!(condition.has_attr_dependency(Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn less_than() {
            let condition = Value::from(Attribute::Debug(0)).less_than(Attribute::Debug(1).into());
            assert!(condition.has_attr_dependency(Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn equal_to() {
            let condition = Value::from(Attribute::Debug(0)).equal_to(Attribute::Debug(1).into());
            assert!(condition.has_attr_dependency(Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn constant() {
            let condition = Condition::from(false);
            // To make sure it's not just returning 0
            assert!(!condition.has_attr_dependency(Attribute::Debug(0)));
        }

        #[test]
        fn and() {
            let condition = attr_condition(0) & attr_condition(1);
            assert!(condition.has_attr_dependency(Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn or() {
            let condition = attr_condition(0) | attr_condition(1);
            assert!(condition.has_attr_dependency(Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(Attribute::Debug(2)));
        }

        #[test]
        fn xor() {
            let condition = attr_condition(0) ^ attr_condition(1);
            assert!(condition.has_attr_dependency(Attribute::Debug(0)));
            assert!(condition.has_attr_dependency(Attribute::Debug(1)));
            assert!(!condition.has_attr_dependency(Attribute::Debug(2)));
        }
    }
}

mod include_dependencies {
    use super::*;

    mod value {
        use super::*;

        #[test]
        fn value() {
            let value = Value::Const(10.into());
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
    }

    mod condition {
        use std::ops::Not;

        use builder::{
            attribute::{Attribute, AttributeDependencies},
            bonus::{Condition, Value},
        };

        fn attr_condition(n: u8) -> Condition {
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

mod base_bonuses {
    use builder::bonus::get_base_bonuses;

    use super::*;

    #[test]
    fn all_base_bonuses_have_base_source() {
        for bonus in get_base_bonuses() {
            assert!(
                matches!(bonus.get_source(), BonusSource::Base),
                "Does not have base bonus: {bonus:?}"
            );
        }
    }
}
