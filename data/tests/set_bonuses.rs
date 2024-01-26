mod utils;
use builder::attribute::AttributeDependencies;
use data::get_set_bonuses;
use itertools::chain;
pub use utils::*;

#[test]
fn bonuses_have_valid_attributes() {
    let valid = valid_attributes()
        .expect("Could not create valid attributes")
        .collect::<Vec<_>>();

    get_set_bonuses()
        .expect("Set Bonuses didn't parse")
        .into_iter()
        .for_each(|set_bonus| {
            set_bonus.bonuses().values().flatten().for_each(|bonus| {
                assert!(
                    valid.contains(bonus.attribute()),
                    "Set Bonus [{}] has bonus to invalid attribute [{}]",
                    set_bonus.name(),
                    bonus.attribute(),
                );

                chain!(
                    bonus.value().get_attr_dependencies(),
                    bonus
                        .condition()
                        .clone()
                        .map(|condition| condition.get_attr_dependencies())
                        .unwrap_or_default()
                )
                .for_each(|attribute| {
                    assert!(
                        valid.contains(&attribute),
                        "Set Bonus [{}] has bonus with invalid dependency [{}]",
                        set_bonus.name(),
                        attribute
                    );
                });
            });
        });
}
