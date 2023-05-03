use crate::build::{
    attribute::{skill::Skill, spell::SpellDamageType, Attribute},
    bonus::{source::Source, types::BonusType, Bonus},
};

macro_rules! bonus {
    ($skill:expr, $attribute:expr, $value:expr) => {
        Bonus::new(
            $attribute,
            BonusType::Skill,
            $value,
            Source::Attribute(Attribute::Skill($skill)),
            None,
        )
    };
}
pub fn get_skill_updates(skill: Skill, value: f32) -> Vec<Bonus> {
    match skill {
        Skill::Heal => vec![
            bonus!(
                Skill::Heal,
                Attribute::SpellPower(SpellDamageType::Positive),
                value
            ),
            bonus!(
                Skill::Heal,
                Attribute::SpellPower(SpellDamageType::Negative),
                value
            ),
        ],
        Skill::Perform => vec![bonus!(
            Skill::Perform,
            Attribute::SpellPower(SpellDamageType::Sonic),
            value
        )],
        Skill::Spellcraft => vec![
            bonus!(
                Skill::Spellcraft,
                Attribute::SpellPower(SpellDamageType::Acid),
                value
            ),
            bonus!(
                Skill::Spellcraft,
                Attribute::SpellPower(SpellDamageType::Cold),
                value
            ),
            bonus!(
                Skill::Spellcraft,
                Attribute::SpellPower(SpellDamageType::Electric),
                value
            ),
            bonus!(
                Skill::Spellcraft,
                Attribute::SpellPower(SpellDamageType::Fire),
                value
            ),
            bonus!(
                Skill::Spellcraft,
                Attribute::SpellPower(SpellDamageType::Force),
                value
            ),
            bonus!(
                Skill::Spellcraft,
                Attribute::SpellPower(SpellDamageType::Light),
                value
            ),
        ],
        _ => Vec::new(),
    }
}
