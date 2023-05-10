mod ability;
pub use ability::*;
mod skill;
pub use skill::*;
mod spell_power;
pub use spell_power::*;
mod spell_school;
pub use spell_school::*;
mod saving_throw;
pub use saving_throw::*;
mod elemental_defenses;
pub use elemental_defenses::*;
mod weapon;
pub use weapon::*;
mod damage_reduction;
pub use damage_reduction::*;
mod offensive;
pub use offensive::*;
mod set_bonus;
pub use set_bonus::*;
mod spell_points;
pub use spell_points::*;
mod healing_amplification;
pub use healing_amplification::*;

#[macro_export]
macro_rules! simple_attribute_enum {
    ($enum_name: ident, ($($id: ident $name: expr),*)) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    };
}
