use builder_core::{
    attribute::{Ability, Attribute, Skill, SpellPower},
    bonus::{Bonus, BonusSource, BonusType},
    breakdown::Breakdowns,
    feat::{Feat, SkillFocus},
};

fn main() {
    let mut breakdown = Breakdowns::new();

    breakdown.insert_bonuses(vec![
        Bonus::new(
            builder_core::attribute::Attribute::Feat(Feat::SkillFocus(SkillFocus::Focus(
                Skill::Heal,
            ))),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(0),
            None,
        ),
        Bonus::new(
            Attribute::Ability(Ability::All),
            BonusType::Stacking,
            8f32,
            BonusSource::Unique(0),
            None,
        ),
        Bonus::new(
            Attribute::Ability(Ability::All),
            BonusType::Stacking,
            8f32,
            BonusSource::Unique(1),
            None,
        ),
        Bonus::new(
            Attribute::Ability(Ability::Wisdom),
            BonusType::Enhancement,
            13f32,
            BonusSource::Unique(0),
            None,
        ),
        Bonus::new(
            Attribute::Ability(Ability::Wisdom),
            BonusType::Insightful,
            7f32,
            BonusSource::Unique(0),
            None,
        ),
        Bonus::new(
            Attribute::SpellPower(SpellPower::Positive),
            BonusType::Quality,
            25f32,
            BonusSource::Unique(3),
            None,
        ),
    ]);

    let ser = ron::to_string(&breakdown).unwrap();

    println!("{}", ser);

    let mut new_breakdowns: Breakdowns = ron::from_str(&ser).unwrap();

    println!(
        "Positive Spell Power: {}",
        new_breakdowns.get_attribute(&Attribute::SpellPower(SpellPower::Positive))
    );
}
