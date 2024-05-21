mod utils;
use builder::attribute::AttributeDependencies;
use data::load_item_sets;
use itertools::chain;
pub use utils::*;

#[test]
fn bonuses_have_valid_attributes() {
    let valid = valid_attributes()
        .expect("Could not create valid attributes")
        .collect::<Vec<_>>();

    load_item_sets()
        .expect("Item Sets didn't parse")
        .into_iter()
        .for_each(|set_bonus| {
            set_bonus.bonuses().values().flatten().for_each(|bonus| {
                assert!(
                    valid.contains(bonus.attribute()),
                    "Item Set [{}] has bonus to invalid attribute [{}]",
                    set_bonus.name(),
                    bonus.attribute(),
                );

                chain!(
                    bonus.value().get_attr_dependencies(),
                    bonus
                        .condition()
                        .cloned()
                        .map(|condition| condition.get_attr_dependencies())
                        .unwrap_or_default()
                )
                .for_each(|attribute| {
                    assert!(
                        valid.contains(&attribute),
                        "Item Set [{}] has bonus with invalid dependency [{}]",
                        set_bonus.name(),
                        attribute
                    );
                });
            });
        });
}
