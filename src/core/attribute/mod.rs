mod skill;
pub use skill::Skill;
mod ability;
pub use ability::Ability;
mod spell_school;
pub use spell_school::SpellSchool;
mod spell_power;
pub use spell_power::SpellPower;
mod healing_amplification;
pub use healing_amplification::HealingAmplification;
mod saving_throw;
pub use saving_throw::SavingThrow;
mod tactics;
pub use tactics::Tactics;

pub enum Attribute {
    BaseAbility(Ability),
    Ability(Ability),
    Skill(Skill),
    SavingThrow(SavingThrow),
    SpellSchoolDC(SpellSchool),
    SpellPower(SpellPower),
    SpellCriticalChance(SpellPower),
    SpellCriticalDamage(SpellPower),
    HealingAmplification(HealingAmplification),
    ArmorClass,
    NaturalArmor,
    MaxDexBonus,
    PhysicalSheltering,
    MagicalSheltering,
    MagicalShelteringCap,
    Fortification,
    Dodge,
    DodgeCap,
    MissileDeflection,
    Incorporeality,
    Displacement,
    HelplessDamageReduction,
    BaseAttackBonus,
    MeleeThreatGeneration,
    RangedThreatGeneration,
    SpellThreatGeneration,
    OffHandAttackChance,
    Doublestrike,
    Doubleshot,
    ImbueDice,
    SneakAttackDice,
    SneakAttack,
    SneakDamage,
    MeleePower,
    RangedPower,
    SecondaryShieldBash,
    DodgeBypass,
    FortificationBypass,
    MissileDeflectionBypass,
    Strikethrough,
    HelplessDamageBonus,
    Tactics(Tactics),
    Health,
}

pub trait Attributable {
    fn into_attribute(self) -> Attribute;
}
