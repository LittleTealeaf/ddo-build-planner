mod value {
    use builder::{attribute::Attribute, bonus::Value};

    #[test]
    fn from_attribute() {
        let value = Value::from(Attribute::Debug(4));
        assert_eq!(value, Value::Attribute(Attribute::Debug(4)));
    }

    #[test]
    fn from_value() {
        let value = Value::from(3f32);

        assert!({
            if let Value::Value(val) = value {
                val == 3f32
            } else {
                false
            }
        });
    }

    mod dependencies {
        use super::*;

        mod has_dependency {
            use builder::{attribute::AttributeDependencies, bonus::Condition};

            use super::*;

            #[test]
            fn value() {
                let value = Value::Value(10f32);

                assert!(!value.has_attr_dependency(Attribute::Debug(0)));
            }

            #[test]
            fn attribute() {
                let value = Value::Attribute(Attribute::Debug(0));

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn sum() {
                let value = Value::Sum(vec![Attribute::Debug(0).into(), 10f32.into()]);

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn product() {
                let value = Value::Product(vec![Attribute::Debug(0).into(), 10f32.into()]);

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn min() {
                let value = Value::Min(vec![Attribute::Debug(0).into(), 10f32.into()]);

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(!value.has_attr_dependency(Attribute::Debug(1)));
            }

            #[test]
            fn max() {
                let value =
                    Value::Max(vec![Attribute::Debug(0).into(), Attribute::Debug(1).into()]);

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
                    condition: Condition::GreaterThan(Attribute::Debug(0).into(), 1f32.into())
                        .into(),
                    if_true: Value::from(Attribute::Debug(1)).into(),
                    if_false: Value::from(Attribute::Debug(2)).into(),
                };

                assert!(value.has_attr_dependency(Attribute::Debug(0)));
                assert!(value.has_attr_dependency(Attribute::Debug(1)));
                assert!(value.has_attr_dependency(Attribute::Debug(2)));
                assert!(!value.has_attr_dependency(Attribute::Debug(3)));
            }
        }

        mod include_dependencies {
            use builder::{attribute::AttributeDependencies, bonus::Condition};

            use super::*;

            #[test]
            fn value() {
                let value = Value::Value(10f32);
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
            fn sum() {
                let value = Value::Sum(vec![Attribute::Debug(0).into(), 10f32.into()]);

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn product() {
                let value = Value::Product(vec![Attribute::Debug(0).into(), 10f32.into()]);

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn min() {
                let value = Value::Min(vec![Attribute::Debug(0).into(), 10f32.into()]);

                let deps = value.get_attr_dependencies();

                assert!(deps.contains(&Attribute::Debug(0)));
                assert!(!deps.contains(&Attribute::Debug(1)));
            }

            #[test]
            fn max() {
                let value =
                    Value::Max(vec![Attribute::Debug(0).into(), Attribute::Debug(1).into()]);

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
                    condition: Condition::GreaterThan(Attribute::Debug(0).into(), 1f32.into())
                        .into(),
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
}
