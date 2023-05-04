#[macro_export]
macro_rules! feats {
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
