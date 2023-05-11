#![allow(dead_code)]
use logic::{
    attribute::{Ability, Attribute, SpellPower},
    bonus::{Bonus, BonusSource, BonusType},
    breakdown::Breakdowns,
};

use crate::logic::{attribute::{Flag, WeaponHand, Skill}, feat::{Feat, SkillFocus}, bonus::Bonuses};

mod logic;
mod utils;

fn main() {
    let mut breakdowns = Breakdowns::new();

    breakdowns.insert_bonuses(Feat::SkillFocus(SkillFocus::Focus(Skill::SpellCraft)).get_bonuses());
    breakdowns.insert_bonuses(Feat::SkillFocus(SkillFocus::Focus(Skill::Heal)).get_bonuses());
    breakdowns.insert_bonuses(Feat::SkillFocus(SkillFocus::SelfSufficient).get_bonuses());
    breakdowns.insert_bonuses(Feat::SkillFocus(SkillFocus::Focus(Skill::SpellCraft)).remove_bonuses());
    println!(
        "{}",
        ron::ser::to_string_pretty(&breakdowns, ron::ser::PrettyConfig::default()).unwrap()
    );


}
