use builder::{breakdowns::Breakdowns, equipment::set_bonus::SetBonus};
use ron::error::SpannedError;

/// Returns a list of set bonuses pulled from the data source
///
/// # Errors
/// Parse Errors (This inicates that there is a parsing error in the data.)
pub fn get_set_bonuses() -> Result<Vec<SetBonus>, SpannedError> {
    include_data!("set_bonuses")
}

/// A trait to implement the `.include_set_bonuses()` to [`Breakdowns`]
pub trait IncludeSetBonuses {
    /// Inserts all set bonuses as dynamic bonuses
    ///
    /// # Errors
    /// Returns a Parsing error if parsing fails
    fn include_set_bonuses(&mut self) -> Result<(), SpannedError>;
}

impl IncludeSetBonuses for Breakdowns {
    fn include_set_bonuses(&mut self) -> Result<(), SpannedError> {
        self.import_dynamic_bonuses(
            get_set_bonuses()?
                .into_iter()
                .map(SetBonus::to_dynamic_bonus),
        );

        Ok(())
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
        assert!(breakdowns.include_set_bonuses().is_ok());
    }
}
