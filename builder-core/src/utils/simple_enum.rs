use std::collections::HashSet;



#[macro_export]
macro_rules! simple_enum {
    ($enum_name: ident, ($($id: ident $name: expr),*)) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug)]
        pub enum $enum_name {
            $($id),*
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$id => $name),*
                })
            }
        }

        #[cfg(test)]
        mod enum_gen_tests {
            use super::*;
            use itertools::Itertools;
            use std::collections::HashSet;

            #[test]
            fn to_string_not_empty() {
                $(
                    assert_ne!(String::from(""), $enum_name::$id.to_string());
                )*
            }

            #[test]
            fn no_duplicate_strings() {
                let mut set: HashSet<String> = vec![$(String::from($name)),*].into_iter().unique().collect();

                $(
                    assert!(set.remove(&$enum_name::$id.to_string()));
                )*

            }
        }
    };
}
