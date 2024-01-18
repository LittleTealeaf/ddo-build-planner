//! Utility functions for loading and saving data
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};

use crate::Message;

pub async fn load_data<T>(path: &str) -> Result<T, DataError>
where
    for<'de> T: Deserialize<'de>,
{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(ron::from_str(contents.as_str())?)
}

pub async fn save_data<T>(path: &str, data: T) -> Result<(), DataError>
where
    T: Serialize + Send + Sync,
{
    let file = File::create(path).await?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(ron::ser::to_string_pretty(&data, PrettyConfig::new())?.as_bytes())
        .await?;
    writer.flush().await?;
    Ok(())
}

pub fn catch_async<T>(function: impl Fn(T) -> Message) -> impl Fn(Result<T, DataError>) -> Message {
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
