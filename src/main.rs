use build::attribute::{Ability, SpellDamageType};
use build::{
    attribute::{Attribute, Skill},
    bonus::{types::BonusType, Bonus},
    breakdowns::Breakdowns,
    feat::Feat,
};

mod build;
mod utils;

fn main() {
    let mut breakdowns = Breakdowns::new();

    breakdowns.insert_attributes(vec![Bonus::new(
        Attribute::Ability(Ability::Intelligence),
        BonusType::Enhancement,
        100f32,
        build::bonus::source::Source::Unique(10),
        None,
    )]);
    breakdowns.insert_attributes(vec![Bonus::new(
        Attribute::Ability(Ability::Wisdom),
        BonusType::Enhancement,
        100f32,
        build::bonus::source::Source::Unique(10),
        None,
    )]);

    breakdowns.insert_attributes(vec![Bonus::new(
        Attribute::SpellPower(SpellDamageType::Positive),
        BonusType::Enhancement,
        142f32,
        build::bonus::source::Source::Unique(10),
        None,
    )]);
    breakdowns.insert_attributes(vec![Bonus::new(
        Attribute::SpellPower(SpellDamageType::Positive),
        BonusType::Insightful,
        72f32,
        build::bonus::source::Source::Unique(10),
        None,
    )]);

    let attributes = breakdowns.get_all_attributes();

    attributes
        .into_iter()
        .for_each(|(attribute, value)| println!("{}: {}", attribute.to_string(), value))
}
