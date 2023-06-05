use builder::{
    attribute::{
        types::{Ability, WeaponHand, WeaponStat},
        Attribute,
    },
    bonus::{Bonus, BonusType},
    compiler::Compiler,
    player_class::PlayerClass, race::Race,
};
use enum_map::Enum;

fn main() {
    let mut compiler = Compiler::default();


    println!("Adding Bonuses");

    compiler.add_bonuses(vec![
        Bonus::new(
            PlayerClass::FavoredSoul.into(),
            BonusType::Stacking,
            10f32.into(),
            0.into(),
            None,
        ),
        Bonus::flag(Race::Gnome.into(), 0.into()),
        Bonus::new(
            Ability::All.into(),
            BonusType::Stacking,
            8f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Intelligence.into(),
            BonusType::Stacking,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Enhancement,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Insightful,
            10f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            (WeaponHand::Main, WeaponStat::Attack).into(),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(Ability::Intelligence).into(),
            2.into(),
            None,
        ),
    ]);

    for (attr, val) in compiler.get_all_attributes() {
        println!("{}: {}", attr, val);
    }


    println!("Sheltering: Both = {}", Attribute::Sheltering(builder::attribute::types::Sheltering::Both).into_usize());
}
