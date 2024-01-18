use builder::{
    attribute::Attribute,
    feat::Feat,
    types::{
        ability::Ability, armor_class::ArmorClass, damage_type::DamageType, flag::Flag,
        heal_amp::HealingAmplification, health::Health, player_class::PlayerClass,
        saving_throw::SavingThrow, sheltering::Sheltering, skill::Skill, spell_points::SpellPoints,
        spell_power::SpellPower, spell_selector::SpellSelector,
        summoned_attribute::SummonedAttribute, toggle::Toggle, weapon_attribute::WeaponAttribute,
    },
};
use itertools::chain;
use utils::enums::StaticOptions;

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
        Ability::get_static().flat_map(|ability| [
            Attribute::Ability(ability),
            Attribute::AbilityModifier(ability)
        ]),
        Skill::get_static().map(Attribute::Skill),
        SavingThrow::get_static().map(Attribute::SavingThrow),
        SpellPower::get_static().flat_map(|sp| {
            [
                Attribute::SpellPower(sp),
                Attribute::SpellCriticalChance(sp),
                Attribute::SpellCriticalDamage(sp),
            ]
        }),
        Toggle::get_static().map(Attribute::Toggle),
        Flag::get_static().map(Attribute::Flag),
        Feat::get_static().map(Attribute::Feat),
        PlayerClass::get_static().map(Attribute::ClassLevel),
        SpellSelector::get_static().flat_map(|selector| {
            [
                Attribute::CasterLevel(selector),
                Attribute::MaxCasterLevel(selector),
                Attribute::SpellDC(selector),
            ]
        }),
        WeaponAttribute::get_static().map(Attribute::Weapon),
        ArmorClass::get_static().map(Attribute::ArmorClass),
        Sheltering::get_static().map(Attribute::Sheltering),
        DamageType::get_static()
            .flat_map(|dt| { [Attribute::Resistance(dt), Attribute::Absorption(dt)] }),
        Health::get_static().map(Attribute::Health),
        SpellPoints::get_static().map(Attribute::SpellPoints),
        SummonedAttribute::get_static().map(Attribute::SummonedAttribute),
        HealingAmplification::get_static().map(Attribute::HealingAmplification),
    )
}
