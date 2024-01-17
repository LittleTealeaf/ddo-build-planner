use builder::equipment::set_bonus::SetBonus;
use iced::Command;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};
use ui::HandleMessage;

use crate::{Editor, Message};

const PATH_SET_BONUSES: &str = "./data/data/set_bonuses.ron";

#[derive(Debug, Clone)]
pub enum DataMessage {
    LoadSetBonuses,
    OnSetBonusesLoaded(Vec<SetBonus>),
    SaveSetBonuses(Box<Message>),
}

impl HandleMessage<DataMessage> for Editor {
    fn handle_message(&mut self, message: DataMessage) -> Command<Self::Message> {
        match message {
            DataMessage::LoadSetBonuses => Command::perform(
                load_data(PATH_SET_BONUSES),
                catch_async(|sets| Message::Data(DataMessage::OnSetBonusesLoaded(sets))),
            ),
            DataMessage::OnSetBonusesLoaded(bonuses) => {
                self.set_bonuses = Some(bonuses);
                Command::none()
            }
            DataMessage::SaveSetBonuses(message) => Command::perform(
                save_data(PATH_SET_BONUSES, self.set_bonuses.clone()),
                catch_async(move |()| *message.clone()),
            ),
        }
    }
}

async fn load_data<T>(path: &str) -> Result<T, DataError>
where
    for<'de> T: Deserialize<'de>,
{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(ron::from_str(contents.as_str())?)
}

async fn save_data<T>(path: &str, data: T) -> Result<(), DataError>
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

fn catch_async<T>(function: impl Fn(T) -> Message) -> impl Fn(Result<T, DataError>) -> Message {
    move |result| match result {
        Ok(val) => function(val),
        Err(err) => Message::Error(format!("{err:?}")),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_bonuses_load() {
        let result = load_data::<Vec<SetBonus>>(format!("../{PATH_SET_BONUSES}").as_str()).await;
        assert!(result.is_ok(), "{result:?}");
    }
}
