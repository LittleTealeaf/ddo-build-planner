use build::{feat::Feat, attribute::Skill};

mod build;

fn main() {
    let feat = Feat::SkillFocus(Skill::Concentration);

    println!("{}: {}", feat.get_name(), feat.get_description());
}
