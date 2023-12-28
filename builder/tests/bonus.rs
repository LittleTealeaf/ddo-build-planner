use builder::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, BonusType, Condition, Value},
};

mod has_dependency {

    use super::*;

    #[test]
    /// Tests that querying a bonus dependency will include the value dependencies
    fn gets_value_dependency() {
        let bonus = Bonus::new(
            Attribute::Debug(0),
            BonusType::Stacking,
            Value::Attribute(Attribute::Debug(1)),
            BonusSource::Debug(0),
            None,
        );

        assert!(bonus.has_attr_dependency(Attribute::Debug(1)));
        assert!(!bonus.has_attr_dependency(Attribute::Debug(2)));
    }

    #[test]
    /// Tests that querying a bonus dependency will include the conditional dependencies
    fn gets_conditional_dependency() {
        let bonus = Bonus::new(
            Attribute::Debug(0),
            BonusType::Stacking,
            Value::Value(10.into()),
            BonusSource::Debug(0),
            Some(Condition::has(Attribute::Debug(1))),
        );

        assert!(bonus.has_attr_dependency(Attribute::Debug(1)));
        assert!(!bonus.has_attr_dependency(Attribute::Debug(2)));
    }

    mod value {
        use super::*;

        #[test]
        fn value() {
            let value = Value::Value(10.into());

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
                Value::from(10f32).into(),
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
                condition: Condition::GreaterThan(Attribute::Debug(0).into(), 1f32.into()).into(),
                if_true: <Box<Value>>::from(Value::from(Attribute::Debug(1))),
                if_false: Value::from(Attribute::Debug(2)).into(),
            };

            assert!(value.has_attr_dependency(Attribute::Debug(0)));
            assert!(value.has_attr_dependency(Attribute::Debug(1)));
            assert!(value.has_attr_dependency(Attribute::Debug(2)));
            assert!(!value.has_attr_dependency(Attribute::Debug(3)));
        }
    }
}

mod include_dependencies {
    use super::*;

    mod value {
        use super::*;

        #[test]
        fn value() {
            let value = Value::Value(10.into());
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
                Value::from(10f32).into(),
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
                condition: Condition::GreaterThan(Attribute::Debug(0).into(), 1f32.into()).into(),
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
}

mod value {
    mod ops {
        use builder::bonus::Value;

        #[test]
        fn add() {
            let value = Value::from(1f32) + Value::from(2f32);
            assert!(matches!(value, Value::Add(_, _)));
        }

        #[test]
        fn sub() {
            let value = Value::from(1f32) - Value::from(1f32);
            assert!(matches!(value, Value::Sub(_, _)));
        }

        #[test]
        fn mul() {
            let value = Value::from(1f32) * Value::from(1f32);
            assert!(matches!(value, Value::Mul(_, _)));
        }

        #[test]
        fn div() {
            let value = Value::from(1f32) / Value::from(1f32);
            assert!(matches!(value, Value::Div(_, _)));
        }

        #[test]
        fn rem() {
            let value = Value::from(1f32) % Value::from(1f32);
            assert!(matches!(value, Value::Rem(_, _)));
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
