//! Handles loading and saving data

use builder::equipment::set_bonus::SetBonus;
use tokio::{fs::File, io::AsyncReadExt};

pub async fn load_set_bonuses() -> Option<Vec<SetBonus>> {
    let mut file = File::open("./data/data/set_bonuses.ron").await.ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.ok()?;

    let parsed_data = ron::from_str(contents.as_str()).ok()?;

    Some(parsed_data)
}
