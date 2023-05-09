#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellPower {
    Universal,
    Acid,
    Light,
    Cold,
    Electric,
    Evil,
    Fire,
    Force,
    Negative,
    Poison,
    Physical,
    Positive,
    Repair,
    Rust,
    Sonic,
}

impl ToString for SpellPower {
    fn to_string(&self) -> String {
        String::from(match self {
            SpellPower::Acid => "Acid",
            SpellPower::Fire => "Fire",
            SpellPower::Electric => "Electric",
            SpellPower::Cold => "Cold",
            SpellPower::Positive => "Positive",
            SpellPower::Negative => "Negative",
            SpellPower::Sonic => "Sonic",
            SpellPower::Force => "Force",
            SpellPower::Light => "Light",
            SpellPower::Repair => "Repair",
            SpellPower::Rust => "Rust",
            SpellPower::Universal => "Universal",
            SpellPower::Evil => "Evil",
            SpellPower::Poison => "Poison",
            SpellPower::Physical => "Physica;",
        })
    }
}

#[macro_export]
macro_rules! spell_power_universal_to_other_helper {
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

#[macro_export]
macro_rules! spell_power_universal_to_others {
    ($attribute: ident, $value: expr) => {
        vec![
            $crate::spell_power_universal_to_other_helper!($attribute, Acid, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Fire, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Electric, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Cold, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Positive, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Negative, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Sonic, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Force, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Light, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Repair, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Rust, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Evil, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Poison, $value),
            $crate::spell_power_universal_to_other_helper!($attribute, Physical, $value),
        ]
    };
}
