use logic::attribute::{Attribute, SpellPower};

mod logic;

fn main() {
    let test = Attribute::SpellPower(SpellPower::Light);
    let test_b = Attribute::SpellCriticalChance(SpellPower::Fire);
}
