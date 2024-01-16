use builder::equipment::set_bonus::SetBonus;
use iced::Command;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};

use crate::Message;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    set_bonuses: Option<Vec<SetBonus>>,
}

#[derive(Clone, Debug)]
// TODO : remove allow after more are added
#[allow(clippy::enum_variant_names)]
pub enum AppStateMessage {
    LoadSetBonuses,
    OnLoadedSetBonuses(Vec<SetBonus>),
    SaveSetBonuses(Box<Message>),
}

impl AppState {
    pub fn update(&mut self, message: AppStateMessage) -> Command<Message> {
        match message {
            AppStateMessage::LoadSetBonuses => Command::perform(
                load_data("./data/data/set_bonuses.ron"),
                |result| match result {
                    Ok(set_bonuses) => {
                        Message::AppState(AppStateMessage::OnLoadedSetBonuses(set_bonuses))
                    }
                    Err(err) => Message::Error(format!("{err:?}")),
                },
            ),
            AppStateMessage::OnLoadedSetBonuses(set_bonuses) => {
                self.set_bonuses = Some(set_bonuses);
                Command::none()
            }
            AppStateMessage::SaveSetBonuses(message) => Command::perform(
                save_data("./data/data/set_bonuses.ron", self.set_bonuses.clone()),
                |result| match result {
                    Err(err) => Message::Error(format!("{err:?}")),
                    _ => *message,
                },
            ),
        }
    }
}

async fn load_data<T>(path: &'static str) -> Result<T, DataError>
where
    for<'de> T: Deserialize<'de>,
{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(ron::from_str(contents.as_str())?)
}

async fn save_data<T>(path: &'static str, data: T) -> Result<(), DataError>
where
    T: Serialize + Sync + Send,
{
    let file = File::open(path).await?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(to_string_pretty(&data, PrettyConfig::new())?.as_bytes())
        .await?;
    writer.flush().await?;
    Ok(())
}

#[derive(Debug)]
/// Describes an error caught from reading or saving data
pub enum DataError {
    /// [`std::io::Error`] Errors
    IO(std::io::Error),
    /// [`ron::de::SpannedError`] Errors
    SpannedError(ron::de::SpannedError),
    /// Generic Ron Error
    Ron(ron::Error),
}

impl From<std::io::Error> for DataError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<ron::de::SpannedError> for DataError {
    fn from(value: ron::de::SpannedError) -> Self {
        Self::SpannedError(value)
    }
}

impl From<ron::Error> for DataError {
    fn from(value: ron::Error) -> Self {
        Self::Ron(value)
    }
}
