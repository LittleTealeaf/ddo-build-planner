mod utils;
use anyhow::Result;
use builder::attribute::AttributeDependencies;
use data::load_item_sets;
use itertools::Itertools;
pub use utils::*;

#[test]
fn bonuses_have_valid_attributes() -> Result<()> {
    let valid = valid_attributes()?.collect_vec();

    for set_bonus in load_item_sets()? {
        for bonus in set_bonus.bonuses().values().flatten() {
            assert!(
                valid.contains(bonus.attribute()),
                "Item Set [{}] has bonus to invalid attribute [{}]",
                set_bonus.name(),
                bonus.attribute(),
            );

            for attribute in bonus.get_attr_dependencies() {
                assert!(
                    valid.contains(&attribute),
                    "Item Set [{}] has bonus with invalid dependency [{}]",
                    set_bonus.name(),
                    attribute
                );
            }
        }
    }

    Ok(())
}
