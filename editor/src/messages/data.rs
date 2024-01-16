use builder::equipment::set_bonus::SetBonus;
use iced::Command;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};

use crate::EditorApp;

use super::{HandleMessage, Message};
/// Messages for loading / saving data
#[derive(Debug, Clone)]
pub enum DataMessage {
    /// Set Bonuses
    SetBonuses(DataIOMessage<Vec<SetBonus>>),
}

impl From<DataMessage> for Message {
    fn from(value: DataMessage) -> Self {
        Self::Data(value)
    }
}

impl HandleMessage for DataMessage {
    fn handle(self, app: &mut EditorApp) -> iced::Command<super::Message> {
        match self {
            Self::SetBonuses(message) => match message {
                DataIOMessage::StartLoad => Command::perform(
                    load_data("./data/data/set_bonuses.ron"),
                    |data| match data {
                        Ok(result) => Self::SetBonuses(DataIOMessage::FinishLoad(result)).into(),
                        Err(err) => Message::Error(format!("{err:?}")),
                    },
                ),
                DataIOMessage::FinishLoad(set_bonuses) => {
                    app.set_bonuses = Some(set_bonuses);
                    Command::none()
                }
                DataIOMessage::StartSave(sets) => Command::perform(
                    save_data("./data/data/set_bonuses.ron", sets),
                    |res: Result<(), DataError>| match res {
                        Err(err) => Message::Error(format!("{err:?}")),
                        _ => Self::SetBonuses(DataIOMessage::FinishSave).into(),
                    },
                ),
                DataIOMessage::FinishSave => {
                    println!("Saved Set Bonuses");
                    Command::none()
                }
            },
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
        .write_all(ron::ser::to_string_pretty(&data, PrettyConfig::new())?.as_bytes())
        .await?;
    writer.flush().await?;
    Ok(())
}

/// Handles generic messages for loading / saving data
#[derive(Debug, Clone)]
pub enum DataIOMessage<T> {
    /// Indicates that loading should start for a specific data point
    StartLoad,
    /// Indicates that the data has finished loading, returning that data
    FinishLoad(T),
    /// Indicates that saving should start
    StartSave(T),
    /// Indicates that saving has finished
    FinishSave,
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
