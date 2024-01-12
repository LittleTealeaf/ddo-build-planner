use builder::equipment::set_bonus::SetBonus;

use crate::util::ParseError;

/// Returns a list of set bonuses pulled from the data source
///
/// # Errors
/// Parse Errors (This inicates that there is a parsing error in the data.)
pub fn get_set_bonuses() -> Result<Vec<SetBonus>, ParseError> {
    include_data!("set_bonuses")
}

#[test]
fn set_bonuses_parses() {
    let data = get_set_bonuses();
    assert!(data.is_ok());
}
