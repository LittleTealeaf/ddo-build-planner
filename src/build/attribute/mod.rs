use self::{
    ability::Ability,
    elemental_defenses::ElementalType,
    flag::Flag,
    saving_throw::SavingThrow,
    skill::Skill,
    spell::{SpellDamageType, SpellSchool}, class_lore::ClassLore,
};

pub mod ability;
pub mod elemental_defenses;
pub mod flag;
pub mod saving_throw;
pub mod skill;
pub mod spell;
pub mod class_lore;
pub(crate) mod macros;

#[derive(PartialEq, Eq, Clone, Hash, Copy)]
pub enum Attribute {
    BaseAbility(Ability),
    Ability(Ability),
    AbilityModifier(Ability),
    Skill(Skill),
    SpellFocus(SpellSchool),
    SpellPower(SpellDamageType),
    SpellCriticalChance(SpellDamageType),
    SpellCriticalDamage(SpellDamageType),
    SavingThrow(SavingThrow),
    ElementalResistance(ElementalType),
    ElementalAbsorption(ElementalType),
    Incorporeality,
    Concealment,
    Dodge,
    MaxDodge,
    MovementSpeed,
    Fortification,
    SpellResistance,
    ArmorClass,
    PhysicalSheltering,
    MagicalSheltering,
    Health,
    HealthScalar,
    UnconsciousRange,
    Vitality,
    SpellPoints,
    HelplessDamageBonus,
    CriticalHitConfirmation,
    CriticalHitDamage,
    FortificationBypass,
    DodgeBypass,
    SneakAttackHit,
    SneakAttackDamage,
    SneakAttackDice,
    Assassinate,
    Stun,
    Trip,
    ImbueDice,
    Strikethrough,
    OneHandedAttackSpeed,
    TwoHandedAttackSpeed,
    TwoWeaponAttackSpeed,
    QuarterstaffAttackSpeed,
    ShieldBashChance,
    OffhandHitChance,
    MeleeThreat,
    SpellThreat,
    BowAttackSpeed,
    ThrownAttackSpeed,
    CrossbowAttackSpeed,
    RepeatingCrossbowAttackSpeed,
    RangedThreat,
    Flag(Flag),
    Attack,
    Damage,
    ClassLore(ClassLore)
}
