#[macro_export]
macro_rules! attributes {
    ($enum: ident, $($id: ident($($param_name: ident: $param_type: ty),*) => ($name: expr, $bonuses: expr)),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $enum {
            $($id($($param_type),*),)*
        }

        impl ToString for $enum {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$id($($param_name),*) => $name,)*
                })
            }
        }

        impl $enum {
            pub fn get_bonuses(&self, value: f32) -> Vec<$crate::build::bonus::Bonus> {
                let source = $crate::build::bonus::source::Source::Attribute(self.clone());
                match self {
                    $(Self::$id($($param_name),*) => $bonuses(value, source),)*
                }
            }
        }
    }
}
