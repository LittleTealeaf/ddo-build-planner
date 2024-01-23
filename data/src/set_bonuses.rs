use builder::{breakdowns::Breakdowns, equipment::set_bonus::SetBonus};

use crate::ParseError;

/// Returns a list of set bonuses pulled from the data source
///
/// # Errors
/// Parse Errors (This inicates that there is a parsing error in the data.)
pub fn get_set_bonuses() -> Result<Vec<SetBonus>, ParseError> {
    include_data!("set_bonuses")
}

/// A trait to implement the `.include_set_bonuses()` to [`Breakdowns`]
pub trait IncludeSetBonuses {
    /// Inserts all set bonuses as dynamic bonuses
    fn include_set_bonuses(&mut self);
}

impl IncludeSetBonuses for Breakdowns {
    fn include_set_bonuses(&mut self) {
        self.import_dynamic_bonuses(
            get_set_bonuses()
                .unwrap()
                .into_iter()
                .map(SetBonus::to_dynamic_bonus),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bonuses_parses() {
        assert!(get_set_bonuses().is_ok());
    }

    #[test]
    fn breakdowns_inserts_bonuses() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.include_set_bonuses();
    }
}
