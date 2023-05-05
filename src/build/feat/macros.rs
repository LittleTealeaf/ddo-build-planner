#[macro_export]
macro_rules! feats_old {
    ($enum_name: ident, $source: ident, $(($id: ident => ($bonuses: expr), $(($type: ty, $name: ident)),*)),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $enum_name {
            $($id($($type),*)),*
        }

        impl $crate::build::bonus::bonuses::Bonuses for $enum_name {
            fn get_bonuses(&self) -> Vec<$crate::build::bonus::Bonus> {
                let source = $crate::build::bonus::source::Source::$source(self.clone());
                match self {
                    $(Self::$id($($name),*) => $bonuses(source),)*
                }
            }
        }
    }
}


#[macro_export]
macro_rules! feats {
    ($enum: ident, $source: ident, $($entry: ident ($($parameter_name: ident: $parameter_type: ty),*) => ($name: expr, $description: expr, $bonuses: expr)),*) => {
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

        impl $crate::build::bonus::bonuses::Bonuses for $enum {
            fn get_bonuses(&self) -> Vec<$crate::build::bonus::Bonus> {
                let source = $crate::build::bonus::source::Source::$source(self.clone());
                match self {
                    $(Self::$entry($($parameter_name),*) => $bonuses(source),)*
                }
            }
        }
    }
}
