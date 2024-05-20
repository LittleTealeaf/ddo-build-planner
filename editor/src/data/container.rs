use core::fmt::Debug;
use std::path::{Path, PathBuf};

use iced::{Application, Command};
use ron::{de::SpannedError, from_str, ser::to_string_pretty};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt, BufWriter},
};
use ui::HandleMessage;
use utils::ron::pretty_config::compact_pretty_config;

use crate::{App, Message};

#[derive(Debug, Clone)]
pub struct DataContainer<T>
where
    T: Debug + Clone,
{
    data: Option<T>,
    modified: bool,
    saving: bool,
    path: PathBuf,
}

use super::DataMessage;

impl<T> DataContainer<T>
where
    T: Clone + Debug,
{
    pub const fn new(path: PathBuf) -> Self {
        Self {
            data: None,
            modified: false,
            saving: false,
            path,
        }
    }

    pub const fn get(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.modified = true;
        self.data.as_mut()
    }

    #[must_use]
    pub const fn modified(&self) -> bool {
        self.modified
    }

    pub const fn saving(&self) -> bool {
        self.saving
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

impl<T> HandleMessage<DataContainerMessage<T>, App> for DataContainer<T>
where
    T: Debug + Clone + Sync + Send + Serialize + for<'de> Deserialize<'de> + 'static,
    DataContainerMessage<T>: Into<DataMessage>,
{
    fn handle_message(
        &mut self,
        message: DataContainerMessage<T>,
    ) -> Command<<App as Application>::Message> {
        match message {
            DataContainerMessage::Load => {
                self.modified = false;
                self.data = None;
                let err_path = self.path.to_str().unwrap().to_owned();

                Command::perform(load_data(self.path.clone()), move |result| match result {
                    Ok(data) => Message::Data(DataContainerMessage::OnLoad(data).into()),
                    Err(err) => Message::Error(format!("Load: {err_path} {err:?}")),
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
                let err_path = self.path.to_str().unwrap().to_owned();

                Command::perform(save_data(self.path.clone(), data.clone()), move |result| {
                    match result {
                        Ok(()) => Message::Data(DataContainerMessage::OnSaved.into()),
                        Err(err) => Message::Error(format!("Save: '{err_path}' {err:?}")),
                    }
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

async fn load_data<T, P>(path: P) -> Result<T, DataError>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path> + Send,
{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let data = from_str(contents.as_str())?;
    Ok(data)
}

async fn save_data<T, P>(path: P, data: T) -> Result<(), DataError>
where
    T: Serialize + Send + Sync,
    P: AsRef<Path> + Send,
{
    let file = File::create(path).await?;
    let mut writer = BufWriter::new(file);
    let serialized = to_string_pretty(&data, compact_pretty_config())?;
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use builder::{bonus::Bonus, debug::DebugValue};
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn save_and_load_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("serialized-file");
        let data = Bonus::new(DebugValue(0), DebugValue(0), 1, DebugValue(0));

        assert!(!file_path.exists());

        save_data(file_path.clone(), data.clone()).await.unwrap();

        assert!(file_path.exists());

        let result = load_data::<Bonus, PathBuf>(file_path).await.unwrap();

        assert_eq!(data, result);
    }
}
