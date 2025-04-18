use core::fmt::Debug;
use std::path::{Path, PathBuf};

use anyhow::Result;
use iced::{Application, Command};
use ron::{from_str, ser::to_string_pretty};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};
use ui::{error, ExecuteMessage, HandleMessage};
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

    pub const fn get_mut(&mut self) -> Option<&mut T> {
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

                let handler = move |result: Result<T>| match result {
                    Ok(data) => Message::Data(DataContainerMessage::OnLoad(data).into()),
                    Err(err) => Message::Log(error!("Load: {err_path} {err:?}")),
                };

                Command::perform(load_data(self.path.clone()), handler)
            }
            DataContainerMessage::OnLoad(data) => {
                self.modified = false;
                self.data = Some(data);

                self.handle_message(DataContainerMessage::Modified)
            }
            DataContainerMessage::Save => {
                let Some(data) = &self.data else {
                    return Command::message(error!("Data is not loaded"));
                };

                self.modified = false;
                self.saving = true;

                let err_path = self.path.to_str().unwrap().to_owned();

                let handler = move |result: Result<()>| match result {
                    Ok(()) => Message::Data(DataContainerMessage::<T>::OnSaved.into()),
                    Err(err) => Message::Log(error!("Save: '{err_path}' {err:?}")),
                };

                Command::perform(save_data(self.path.clone(), data.clone()), handler)
            }
            DataContainerMessage::OnSaved => {
                self.saving = false;
                Command::none()
            }
            DataContainerMessage::Modified => {
                self.modified = true;
                self.handle_message(DataContainerMessage::<T>::Save)
            }
        }
    }
}

async fn load_data<T, P>(path: P) -> Result<T>
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

async fn save_data<T, P>(path: P, data: T) -> Result<()>
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use builder::{bonus::Bonus, debug::DebugValue};
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn save_and_load_file() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("serialized-file");
        let data = Bonus::new(DebugValue(0), DebugValue(0), 1, DebugValue(0));

        assert!(!file_path.exists());

        save_data(file_path.clone(), data.clone()).await?;

        assert!(file_path.exists());

        let result = load_data::<Bonus, PathBuf>(file_path).await?;

        assert_eq!(data, result);

        Ok(())
    }
}
