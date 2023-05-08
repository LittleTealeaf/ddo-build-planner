#[macro_export]
macro_rules! feats {
    ($enum: ident, $($entry: ident ($($parameter_name: ident: $parameter_type: ty),*) => ($name: expr, $description: expr, $bonuses: expr)),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $enum {
            $($entry($($parameter_type),*)),*
        }


        impl $enum {
            pub fn get_name(&self) -> String {
                match self {
                    $(Self::$entry($($parameter_name),*) => String::from($name),)*
                }
            }

            pub fn get_description(&self) -> String {
                match self {
                    $(Self::$entry($($parameter_name),*) => String::from($description),)*
                }
            }
        }


        impl ToString for $enum {
            fn to_string(&self) -> String {
                self.get_name()
            }
        }

        impl $crate::build::bonus::Bonuses for $enum {
            fn get_bonuses(&self) -> Vec<$crate::build::bonus::Bonus> {
                let source = $crate::build::bonus::Source::Feat(self.clone().into());
                match self {
                    $(Self::$entry($($parameter_name),*) => $bonuses(source),)*
                }
            }
        }
    }
}
