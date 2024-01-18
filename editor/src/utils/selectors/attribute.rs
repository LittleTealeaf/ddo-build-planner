use builder::{
    attribute::Attribute,
    feat::Feat,
    types::{
        ability::Ability, armor_class::ArmorClass, damage_type::DamageType, flag::Flag,
        health::Health, player_class::PlayerClass, saving_throw::SavingThrow,
        sheltering::Sheltering, skill::Skill, spell_points::SpellPoints, spell_power::SpellPower,
        spell_selector::SpellSelector, summoned_attribute::SummonedAttribute, toggle::Toggle,
        weapon_attribute::WeaponAttribute, heal_amp::HealingAmplification,
    },
};
use itertools::chain;
use utils::all::AllStatic;

pub enum AttributeOptions {
    Attribute(Attribute),
    SetBonus,
}

impl AttributeOptions {
    pub fn options() -> impl Iterator<Item = Self> {
        chain!([Self::SetBonus], all_attributes().map(Self::Attribute))
    }
}

pub fn all_attributes() -> impl Iterator<Item = Attribute> {
    chain!(
        [
            Attribute::Dummy,
            Attribute::SpellResistance,
            Attribute::SpellPenetration,
            Attribute::TotalCharacterLevel,
            Attribute::ArmorCheckPenalty
        ],
        Ability::all().flat_map(|ability| [
            Attribute::Ability(ability),
            Attribute::AbilityModifier(ability)
        ]),
        Skill::all().map(Attribute::Skill),
        SavingThrow::all().map(Attribute::SavingThrow),
        SpellPower::all().flat_map(|sp| {
            [
                Attribute::SpellPower(sp),
                Attribute::SpellCriticalChance(sp),
                Attribute::SpellCriticalDamage(sp),
            ]
        }),
        Toggle::all().map(Attribute::Toggle),
        Flag::all().map(Attribute::Flag),
        Feat::all().map(Attribute::Feat),
        PlayerClass::all().map(Attribute::ClassLevel),
        SpellSelector::all().flat_map(|selector| {
            [
                Attribute::CasterLevel(selector),
                Attribute::MaxCasterLevel(selector),
                Attribute::SpellDC(selector),
            ]
        }),
        WeaponAttribute::all().map(Attribute::Weapon),
        ArmorClass::all().map(Attribute::ArmorClass),
        Sheltering::all().map(Attribute::Sheltering),
        DamageType::all().flat_map(|dt| { [Attribute::Resistance(dt), Attribute::Absorption(dt)] }),
        Health::all().map(Attribute::Health),
        SpellPoints::all().map(Attribute::SpellPoints),
        SummonedAttribute::all().map(Attribute::SummonedAttribute),
        HealingAmplification::all().map
            (Attribute::HealingAmplification),
    )
}
