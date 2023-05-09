use crate::logic::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
};

use super::SpellPower;

macro_rules! set_bonuses {
    ($($id: ident, $name: expr => ($($count: expr => $bonuses: expr),*)),*) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub enum SetBonus {
            $($id),*
        }

        impl ToString for SetBonus {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$id => $name),*
                })
            }
        }

        impl SetBonus {
            pub fn get_bonuses(&self, value: f32) -> Option<Vec<$crate::logic::bonus::Bonus>> {
                match self {
                    $(Self::$id => {
                        let mut vec = vec![
                            Bonus::new(Attribute::Dummy, BonusType::Stacking, 0f32, BonusSource::Attribute(Attribute::SetBonus(SetBonus::$id)), None)
                        ];
                        $(
                            if value >= $count {
                                for item in $bonuses {
                                    vec.push(item)
                                }
                            }
                        )*
                        if vec.len() > 0 {Some(vec)} else {None}
                    }),*
                }
            }
        }
    }
}

set_bonuses!(
    LegendaryEldersKnowledge, "Legendary Elder's Knowledge" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Universal), BonusType::Artifact, 6f32, BonusSource::Attribute(Attribute::SetBonus(SetBonus::LegendaryEldersKnowledge)), None),
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Universal), BonusType::Legendary, 15f32, BonusSource::Attribute(Attribute::SetBonus(SetBonus::LegendaryEldersKnowledge)), None),
        ]
    )
);
