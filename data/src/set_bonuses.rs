use builder::{breakdowns::Breakdowns, equipment::set_bonus::SetBonus};

use crate::util::ParseError;

/// Returns a list of set bonuses pulled from the data source
///
/// # Errors
/// Parse Errors (This inicates that there is a parsing error in the data.)
pub fn get_set_bonuses() -> Result<Vec<SetBonus>, ParseError> {
    include_data!("set_bonuses")
}

/// A trait to implement the `.include_set_bonuses()` to [`Breakdowns`]
pub trait ImportSetBonuses {
    /// Inserts all set bonuses as dynamic bonuses
    fn import_set_bonuses(&mut self);
}

impl ImportSetBonuses for Breakdowns {
    fn import_set_bonuses(&mut self) {
        self.import_dynamic_bonuses(
            get_set_bonuses()
                .unwrap()
                .into_iter()
                .map(SetBonus::to_dynamic_bonus),
        );
    }
}

#[test]
fn set_bonuses_parses() {
    let data = get_set_bonuses();
    assert!(data.is_ok());
}
