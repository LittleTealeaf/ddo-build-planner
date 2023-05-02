use build::{bonus::bonuses::Bonuses, feat::heroic::skill::SkillFeat};

mod build;

fn main() {
    let value = SkillFeat::Athletic.get_bonuses();
}
