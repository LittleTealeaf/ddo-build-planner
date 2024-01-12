//! Application Starting Point

use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusTemplate, BonusType},
    breakdowns::Breakdowns,
    equipment::set_bonus::SetBonus,
    types::{
        ability::Ability, armor_class::ArmorClass, sheltering::Sheltering,
        spell_selector::SpellSelector,
    },
};
use data::get_set_bonuses;
use ron::ser::PrettyConfig;

fn main() {
    // println!("{:?}", get_set_bonuses().unwrap());

    let mut breakdowns = Breakdowns::new();

    breakdowns.insert_bonuses(
        get_set_bonuses()
            .unwrap()
            .into_iter()
            .flat_map(SetBonus::to_bonuses),
    );

    breakdowns.insert_bonus(Bonus::new(
        Attribute::SetBonus("Might of the Abashai".to_string()),
        BonusType::Stacking,
        5,
        BonusSource::Custom(0),
        None,
    ));

    println!(
        "{}",
        ron::ser::to_string_pretty(&breakdowns, PrettyConfig::new()).unwrap()
    );

    // let mut set_bonus = SetBonus::new("Might of the Abashai".into());
    //
    // let bonuses = set_bonus.bonuses_mut();
    //
    // bonuses.insert(
    //     3,
    //     vec![
    //         BonusTemplate::new(ArmorClass::NaturalArmor, BonusType::Profane, 3, None),
    //         BonusTemplate::new(Ability::All, BonusType::Profane, 1, None),
    //         BonusTemplate::new(
    //             Attribute::SpellDC(SpellSelector::All),
    //             BonusType::Profane,
    //             1,
    //             None,
    //         ),
    //     ],
    // );
    // bonuses.insert(
    //     5,
    //     vec![
    //         BonusTemplate::new(ArmorClass::NaturalArmor, BonusType::Profane, 5, None),
    //         BonusTemplate::new(Ability::All, BonusType::Profane, 2, None),
    //         BonusTemplate::new(
    //             Attribute::SpellDC(SpellSelector::All),
    //             BonusType::Profane,
    //             2,
    //             None,
    //         ),
    //     ],
    // );
    //
    // println!("{}", ron::to_string(&set_bonus).unwrap());
    //
    // // let mut breakdowns = Breakdowns::new();
    // // breakdowns.insert_bonuses([
    // //     Bonus::new(
    // //         Sheltering::Physical,
    // //         BonusType::Stacking,
    // //         100,
    // //         BonusSource::Custom(0),
    // //         None,
    // //     ),
    // //     Bonus::new(
    // //         Ability::All,
    // //         BonusType::Stacking,
    // //         30,
    // //         BonusSource::Custom(0),
    // //         None,
    // //     ),
    // // ]);
    // // println!("{breakdowns:?}");
}
