macro_rules! attributes {
    ($enum_name: ident, $value: ident, $($name: ident($($param_name: ident: $param_type: ty),*) => ($string: expr, $description: expr, $bonuses: expr, $clones: expr))*) => {
        /// Describes different attributes that a character can have.  Each bonus may give one or more of these attributes.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, enum_map::Enum)]
        pub enum $enum_name {
            $(
                #[doc = $description]
                $name($($param_type),*)
            ),*
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                match self {
                    $($enum_name::$name($($param_name),*) => $string),*
                }
            }
        }

        impl $enum_name {
            /// Gets any subsidiary bonuses that an attribute might have.
            pub fn get_attribute_bonuses(&self, $value: f32) -> Option<Vec<Bonus>> {
                match self {
                    $($enum_name::$name($($param_name),*) => $bonuses),*
                }
            }

            /// Gets any clones that an attribute might split into.
            pub fn get_attribute_clones(&self) -> Option<Vec<$enum_name>> {
                match self {
                    $($enum_name::$name($($param_name),*) => $clones),*
                }
            }
        }
    }
}
