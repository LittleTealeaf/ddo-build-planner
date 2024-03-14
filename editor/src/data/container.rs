use core::fmt::Debug;

use iced::{Application, Command};
use ron::{
    de::SpannedError,
    from_str,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt, BufWriter},
};
use ui::HandleMessage;

use crate::{Editor, Message};

use super::DataMessage;

#[derive(Debug, Clone)]
pub struct DataContainer<T>
where
    T: Debug + Clone,
{
    pub data: Option<T>,
    pub modified: bool,
    pub saving: bool,
    path: &'static str,
}

impl<T> DataContainer<T>
where
    T: Clone + Debug,
{
    pub const fn new(path: &'static str) -> Self {
        Self {
            data: None,
            modified: false,
            saving: false,
            path,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataContainerMessage<T>
where
    T: Debug + Clone,
{
    Load,
    OnLoad(T),
    Save,
    OnSaved,
    Modified,
}

impl<T> HandleMessage<DataContainerMessage<T>, Editor> for DataContainer<T>
where
    T: Debug + Clone + Sync + Send + Serialize + 'static + for<'de> serde::Deserialize<'de>,
    DataMessage: From<DataContainerMessage<T>>,
{
    fn handle_message(
        &mut self,
        message: DataContainerMessage<T>,
    ) -> Command<<Editor as Application>::Message> {
        match message {
            DataContainerMessage::Load => {
                self.modified = false;
                self.data = None;
                Command::perform(load_data(self.path), |result| match result {
                    Ok(value) => DataMessage::from(DataContainerMessage::<T>::OnLoad(value)).into(),
                    Err(error) => Message::Error(format!("{error:?}")),
                })
            }
            DataContainerMessage::OnLoad(data) => {
                self.modified = false;
                self.data = Some(data);
                Command::none()
            }
            DataContainerMessage::Save => self.data.as_ref().map_or_else(Command::none, |data| {
                self.modified = false;
                self.saving = true;
                Command::perform(save_data(self.path, data.clone()), |result| match result {
                    Ok(()) => DataMessage::from(DataContainerMessage::<T>::OnSaved).into(),
                    Err(error) => Message::Error(format!("{error:?}")),
                })
            }),
            DataContainerMessage::OnSaved => {
                self.saving = false;
                Command::none()
            }
            DataContainerMessage::Modified => {
                self.modified = true;
                Command::none()
            }
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
    Ok(from_str(contents.as_str())?)
}

async fn save_data<T>(path: &str, data: T) -> Result<(), DataError>
where
    T: Serialize + Send + Sync,
{
    let file = File::create(path).await?;
    let mut writer = BufWriter::new(file);
    let serialized = to_string_pretty(&data, PrettyConfig::new())?;
    writer.write_all(serialized.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

#[derive(Debug)]
enum DataError {
    IO(io::Error),
    SpannedError(SpannedError),
    Ron(ron::Error),
}

impl From<io::Error> for DataError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<SpannedError> for DataError {
    fn from(value: SpannedError) -> Self {
        Self::SpannedError(value)
    }
}

impl From<ron::Error> for DataError {
    fn from(value: ron::Error) -> Self {
        Self::Ron(value)
    }
}
