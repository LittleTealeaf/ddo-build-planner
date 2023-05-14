#[macro_export]
macro_rules! attributes {
    ($enum_name: ident, $value: ident, $($name: ident($($param_name: ident: $param_type: ty),*) => ($string: expr, $bonuses: expr, $clones: expr))*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
        pub enum $enum_name {
            $($name($($param_type),*)),*
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                match self {
                    $($enum_name::$name($($param_name),*) => $string),*
                }
            }
        }

        impl $enum_name {
            pub fn get_attribute_bonuses(&self, $value: f32) -> Option<Vec<$crate::bonus::Bonus>> {
                match self {
                    $($enum_name::$name($($param_name),*) => $bonuses),*
                }
            }

            pub fn get_attribute_clones(&self) -> Option<Vec<$enum_name>> {
                match self {
                    $($enum_name::$name($($param_name),*) => $clones),*
                }
            }
        }
    }
}
