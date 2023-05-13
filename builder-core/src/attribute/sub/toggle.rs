use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

use super::Flag;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Debug)]
pub enum Toggle {
    Blocking,
    Intimidating,
    AttackingEvilCreatures,
}

impl Toggle {
    pub fn get_toggled_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Intimidating => Some(vec![
                Bonus::new(
                    Attribute::ThreatMultipler(super::Threat::Melee),
                    BonusType::Stacking,
                    4f32,
                    crate::bonus::BonusSource::Attribute(Attribute::Flag(Flag::Toggle(
                        Toggle::Intimidating,
                    ))),
                    None,
                ),
                Bonus::new(
                    Attribute::ThreatMultipler(super::Threat::Ranged),
                    BonusType::Stacking,
                    1f32,
                    crate::bonus::BonusSource::Attribute(Attribute::Flag(Flag::Toggle(
                        Toggle::Intimidating,
                    ))),
                    None,
                ),
                Bonus::new(
                    Attribute::ThreatMultipler(super::Threat::Spell),
                    BonusType::Stacking,
                    1f32,
                    crate::bonus::BonusSource::Attribute(Attribute::Flag(Flag::Toggle(
                        Toggle::Intimidating,
                    ))),
                    None,
                ),
            ]),
            _ => None,
        }
    }
}

impl ToString for Toggle {
    fn to_string(&self) -> String {
        String::from(match self {
            Toggle::Blocking => "Blocking",
            Toggle::Intimidating => "Intimidating",
            Toggle::AttackingEvilCreatures => "Attacking Evil Creatures"
        })
    }
}
