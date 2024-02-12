use std::io;

use ron::{
    de, from_str,
    ser::{to_string_pretty, PrettyConfig},
};
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
    Ok(from_str(contents.as_str())?)
}

pub async fn save_data<T>(path: &str, data: T) -> Result<(), DataError>
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

pub fn catch_async<T, F, M>(function: F) -> impl Fn(Result<T, DataError>) -> Message
where
    F: Fn(T) -> M,
    M: Into<Message>,
{
    move |result| match result {
        Ok(val) => function(val).into(),
        Err(err) => Message::Error(format!("{err:?}")),
    }
}

#[derive(Debug)]
pub enum DataError {
    IO(io::Error),
    SpannedError(de::SpannedError),
    Ron(ron::Error),
}

impl From<io::Error> for DataError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<de::SpannedError> for DataError {
    fn from(value: de::SpannedError) -> Self {
        Self::SpannedError(value)
    }
}

impl From<ron::Error> for DataError {
    fn from(value: ron::Error) -> Self {
        Self::Ron(value)
    }
}
