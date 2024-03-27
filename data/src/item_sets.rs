use builder::{breakdowns::Breakdowns, equipment::set_bonus::ItemSet};
use ron::error::SpannedError;

/// Returns a list of item sets pulled from the data source
///
/// # Errors
/// Parse Errors (This inicates that there is a parsing error in the data.)
pub fn get_item_sets() -> Result<Vec<ItemSet>, SpannedError> {
    include_data!("item_sets")
}

/// A trait to implement the `.import_item_sets()` to [`Breakdowns`]
pub trait ImportItemSets {
    /// Inserts all item sets as dynamic bonuses
    ///
    /// # Errors
    /// Returns a Parsing error if parsing fails
    fn import_item_sets(&mut self) -> Result<(), SpannedError>;
}

impl ImportItemSets for Breakdowns {
    fn import_item_sets(&mut self) -> Result<(), SpannedError> {
        self.import_dynamic_bonuses(get_item_sets()?.into_iter().map(ItemSet::to_dynamic_bonus));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_sets_parses() {
        get_item_sets().expect("Expected Item Sets to Parse");
    }

    #[test]
    fn breakdowns_inserts_bonuses() {
        let mut breakdowns = Breakdowns::new();
        breakdowns
            .import_item_sets()
            .expect("Expected Item Sets to be imported");
    }
}
