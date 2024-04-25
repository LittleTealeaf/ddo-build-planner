use builder::attribute::Attribute;
use data::{load_item_sets, ParseError};
use itertools::chain;
use utils::enums::StaticOptions;

/// Returns an iterator of all valid attributes
///
/// # Errors
/// Returns an error if there is a parsing error
pub fn valid_attributes() -> Result<impl Iterator<Item = Attribute>, ParseError> {
    Ok(chain!(
        Attribute::get_static(),
        load_item_sets()?
            .into_iter()
            .map(|set_bonus| { Attribute::ItemSet(set_bonus.name().clone()) })
    ))
}
