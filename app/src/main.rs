use builder_core::{bonus::{BonusSource, BonusType}, attribute::{Attribute, Ability, CasterLevel}, feat::Tome};
use enum_map::Enum;

fn main() {
    let value = BonusSource::Feat(builder_core::feat::Feat::Tome(Tome::SpellPower));
    println!("{}", value.into_usize());
}
