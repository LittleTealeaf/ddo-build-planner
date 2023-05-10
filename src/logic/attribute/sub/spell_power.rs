use crate::simple_enum;

simple_enum!(SpellPower, (Universal "Universal", Acid "Acid", Light "Light", Cold "Cold", Electric "Electric", Evil "Evil", Fire "Fire", Force "Force", Negative "Negative", Poison "Poison", Positive "Positive", Repair "Repair", Rust "Rust", Sonic "Sonic", Potency "Potency"));

pub const POTENCY_CLONED_ATTRIBUTES: [SpellPower; 13] = [
    SpellPower::Acid,
    SpellPower::Light,
    SpellPower::Cold,
    SpellPower::Electric,
    SpellPower::Evil,
    SpellPower::Fire,
    SpellPower::Force,
    SpellPower::Negative,
    SpellPower::Poison,
    SpellPower::Positive,
    SpellPower::Repair,
    SpellPower::Rust,
    SpellPower::Sonic,
];

#[macro_export]
macro_rules! spell_power_universal_to_others {
    ($attribute: ident, $value: expr) => {
        vec![
            $crate::spell_power_universal_to_others!($attribute, Acid, $value),
            $crate::spell_power_universal_to_others!($attribute, Fire, $value),
            $crate::spell_power_universal_to_others!($attribute, Electric, $value),
            $crate::spell_power_universal_to_others!($attribute, Cold, $value),
            $crate::spell_power_universal_to_others!($attribute, Positive, $value),
            $crate::spell_power_universal_to_others!($attribute, Negative, $value),
            $crate::spell_power_universal_to_others!($attribute, Sonic, $value),
            $crate::spell_power_universal_to_others!($attribute, Force, $value),
            $crate::spell_power_universal_to_others!($attribute, Light, $value),
            $crate::spell_power_universal_to_others!($attribute, Repair, $value),
            $crate::spell_power_universal_to_others!($attribute, Rust, $value),
            $crate::spell_power_universal_to_others!($attribute, Evil, $value),
            $crate::spell_power_universal_to_others!($attribute, Poison, $value),
        ]
    };
    ($attribute: ident, $spell_power: ident, $value: expr) => {
        $crate::logic::bonus::Bonus::new(
            Attribute::$attribute($crate::logic::attribute::SpellPower::$spell_power),
            $crate::logic::bonus::BonusType::Stacking,
            $value,
            $crate::logic::bonus::BonusSource::Attribute(
                $crate::logic::attribute::Attribute::$attribute(
                    $crate::logic::attribute::SpellPower::Universal,
                ),
            ),
            None,
        )
    };
}
