use builder::attribute::Attribute;
use data::{get_set_bonuses, ParseError};
use itertools::chain;
use utils::enums::StaticOptions;

/// Returns an iterator of all valid attributes
///
/// # Errors
/// Returns an error if there is a parsing error
pub fn valid_attributes() -> Result<impl Iterator<Item = Attribute>, ParseError> {
    Ok(chain!(
        Attribute::get_static(),
        get_set_bonuses()?
            .into_iter()
            .map(|set_bonus| { Attribute::SetBonus(set_bonus.name().clone()) })
    ))
}
