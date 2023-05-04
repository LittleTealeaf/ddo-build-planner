use crate::build::{
    attribute::{skill::Skill, Attribute},
    bonus::{bonuses::Bonuses, source::Source},
    bonus::{types::BonusType, Bonus},
};

// TODO: Maybe, we add in the "generics" at the end

macro_rules! feats {
    ($name: ident, $(($id: ident => ($bonuses: expr), $(($type: ty, $n: ident)),*)),*) => {
        enum $name {
            $($id($($type),*)),*
        }

        impl $crate::build::bonus::bonuses::Bonuses for $name {
            fn get_bonuses(&self) -> Vec<$crate::build::bonus::Bonus> {
                match self {
                    $(Self::$id($($n),*) => $bonuses,)*
                }
            }
        }
    }
}

feats!(
MyFeats,
(Test => (vec![
          Bonus::new(Attribute::Skill(skill.clone()), BonusType::Stacking, 2.0, Source::Unique(3), None)

]), (Skill, skill)
)
);

fn test() {
    MyFeats::Test(Skill::Heal).get_bonuses();
}
