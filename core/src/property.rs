use itertools::chain;

use crate::{
    attribute::Attribute,
    traits::IterValues,
    types::{
        ability::Ability, skill::Skill, spell_damage_type::SpellDamageType,
        spell_school::SpellSchool, weapon_attribute::WeaponAttribute, weapon_slot::WeaponSlot,
    },
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
pub enum Property {
    #[display("Attribute: {_0}")]
    Attribute(Attribute),
    #[display("All Ability Scores")]
    AllAbilityScores,
    #[display("All Skills")]
    AllSkills,
    #[display("{_0} Skills")]
    AbilitySkills(Ability),
    #[display("{_0}")]
    Weapon(WeaponAttribute),
    #[display("Potency")]
    Potency,
    SpellDC,
}

impl Property {
    pub fn into_attributes(self) -> Vec<Attribute> {
        match self {
            Self::Weapon(attribute) => WeaponSlot::values()
                .map(|slot| Attribute::Weapon(attribute, slot))
                .collect(),
            Self::Attribute(attribute) => vec![attribute],
            Self::AllAbilityScores => Ability::values()
                .map(Attribute::AbilityScore)
                .collect::<Vec<_>>(),
            Self::AllSkills => Skill::values().map(Attribute::Skill).collect(),
            Self::Potency => Vec::from(SpellDamageType::SPELL_POWERS.map(Attribute::SpellPower)),
            Self::AbilitySkills(ability) => Skill::values()
                .filter(|skill| skill.ability() == ability)
                .map(Attribute::Skill)
                .collect(),
            Self::SpellDC => SpellSchool::values().map(SpellSchool::spell_dc).collect(),
        }
    }
}

impl IterValues for Property {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            WeaponAttribute::values().map(Self::Weapon),
            Attribute::values().map(Self::Attribute),
            Ability::values().map(Self::AbilitySkills),
            [
                Self::AllAbilityScores,
                Self::AllSkills,
                Self::SpellDC,
                Self::Potency,
            ]
        )
    }
}
