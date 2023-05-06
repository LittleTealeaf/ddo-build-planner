mod ability;
pub use ability::*;
mod skill;
pub use skill::*;
mod flag;
pub use flag::*;
mod class_lore;
pub use class_lore::*;
mod spells;
pub use spells::*;
mod saving_throw;
pub use saving_throw::*;
mod elemental;
pub use elemental::*;


#[macro_export]
macro_rules! attribute_subtype {
    ($enum: ident, $(($identifier: ident $name: expr)),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $enum {
            $($identifier,)*
        }


        impl ToString for $enum {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$identifier => $name,)*
                })
            }
        }
    };
}
