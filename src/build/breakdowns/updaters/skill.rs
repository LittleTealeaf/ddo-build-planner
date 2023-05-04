use crate::build::{
    attribute::{skill::Skill, spell::SpellDamageType, Attribute},
    bonus::{source::Source, types::BonusType, Bonus},
};

macro_rules! bonus {
    ($skill:ident, $attribute:expr, $value:expr) => {
        Bonus::new(
            $attribute,
            BonusType::Skill,
            $value,
            Source::Attribute(Attribute::Skill(Skill::$skill)),
            None,
        )
    };
}

macro_rules! spell_power {
    ($skill: ident, $spellpower: ident, $value: expr) => {
        bonus!(
            $skill,
            Attribute::SpellPower(SpellDamageType::$spellpower),
            $value
        )
    };
}

pub fn get_skill_updates(skill: Skill, value: f32) -> Vec<Bonus> {
    match skill {
        Skill::Heal => vec![
            spell_power!(Heal, Positive, value),
            spell_power!(Heal, Negative, value),
        ],
        Skill::Perform => vec![spell_power!(Perform, Sonic, value)],
        Skill::Spellcraft => vec![
            spell_power!(Spellcraft, Acid, value),
            spell_power!(Spellcraft, Cold, value),
            spell_power!(Spellcraft, Electric, value),
            spell_power!(Spellcraft, Fire, value),
            spell_power!(Spellcraft, Force, value),
            spell_power!(Spellcraft, Light, value),
        ],
        _ => Vec::new(),
    }
}
