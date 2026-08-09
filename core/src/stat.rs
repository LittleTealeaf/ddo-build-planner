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
pub enum Stat {
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

impl Stat {
    pub fn into_attributes(self) -> Vec<Attribute> {
        match self {
            Self::Weapon(attribute) => WeaponSlot::values()
                .map(|slot| Attribute::Weapon(attribute, slot))
                .collect(),
            Self::Attribute(attribute) => vec![attribute],
            Self::AllAbilityScores => Vec::from(Ability::VALUES.map(Attribute::AbilityScore)),
            Self::AllSkills => Vec::from(Skill::VALUES.map(Attribute::Skill)),
            Self::Potency => Vec::from(SpellDamageType::SPELL_POWERS.map(Attribute::SpellPower)),
            Self::AbilitySkills(ability) => Skill::values()
                .filter(|skill| skill.ability() == ability)
                .map(Attribute::Skill)
                .collect(),
            Self::SpellDC => Vec::from(SpellSchool::VALUES.map(SpellSchool::spell_dc)),
        }
    }
}
