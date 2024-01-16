use builder::equipment::set_bonus::SetBonus;
use tokio::{fs::File, io::AsyncReadExt};

use super::DataError;

/// Handles loading and saving of set bonus data
pub struct SetBonusData;

impl SetBonusData {

    /// Reads the file and returns the list of set bonuses
    ///
    /// # Errors
    /// Returns errors if IO or Parsing errors occurred
    pub async fn load() -> Result<Vec<SetBonus>, DataError> {
        let mut file = File::open("./data/data/set_bonuses.ron").await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        Ok(ron::from_str(contents.as_str())?)
    }
}
