use std::path::{Path, PathBuf};

use anyhow::Result;
use iced::Task;
use iced_ui::{model::Model, signal::Signal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct DataBlock<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Sized,
{
    data: Option<T>,
    path: PathBuf,
    is_saving: bool,
    save_again: bool,
}

#[derive(Clone, Debug)]
pub enum DataBlockMsg<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Sized,
{
    StartLoad,
    StartSave,
    DataLoaded(Box<T>),
    DataSaved,
    Error(String),
}

impl<T> DataBlock<T>
where
    T: Serialize + for<'a> Deserialize<'a> + 'static,
{
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            is_saving: false,
            save_again: false,
            data: None,
            path: path.as_ref().to_path_buf(),
        }
    }

    #[must_use]
    pub const fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub const fn data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    pub fn load(&self) -> Task<DataBlockMsg<T>> {
        let path = self.path.clone();
        Task::future(async move {
            match read_data(path).await {
                Ok(data) => DataBlockMsg::DataLoaded(Box::new(data)),
                Err(error) => DataBlockMsg::Error(error.to_string()),
            }
        })
    }
}

impl<T> DataBlock<Vec<T>>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flatten()
    }
}

impl<T> Model for DataBlock<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Sized + 'static,
{
    type Message = DataBlockMsg<T>;
    type Context<'a>
        = ()
    where
        Self: 'a;

    type OutMessage = ();

    fn update(
        &mut self,
        message: Self::Message,
        (): Self::Context<'_>,
    ) -> Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            DataBlockMsg::StartLoad => Ok(Signal::Task(self.load())),
            DataBlockMsg::StartSave => {
                if self.is_saving {
                    self.save_again = true;
                    return Ok(Signal::Done);
                }
                let Some(data) = &self.data else {
                    return Ok(Signal::Done);
                };
                self.is_saving = true;
                self.save_again = false;

                let serialized = ron::to_string(data)?;
                let path = self.path.clone();
                let future = async move {
                    let result = async_fs::write(path, serialized.as_bytes()).await;
                    match result {
                        Ok(()) => DataBlockMsg::DataSaved,
                        Err(error) => DataBlockMsg::Error(error.to_string()),
                    }
                };

                Ok(Signal::Task(Task::future(future)))
            }
            DataBlockMsg::DataLoaded(data) => {
                self.data = Some(*data);
                Ok(Signal::Message(DataBlockMsg::StartSave))
            }
            DataBlockMsg::DataSaved => {
                self.is_saving = false;
                if self.save_again {
                    self.save_again = false;
                    Ok(Signal::Message(DataBlockMsg::StartSave))
                } else {
                    Ok(Signal::Done)
                }
            }
            DataBlockMsg::Error(error) => Err(anyhow::anyhow!("Error: {error}")),
        }
    }
}

async fn read_data<P, T>(path: P) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
    P: AsRef<Path>,
{
    let string = async_fs::read_to_string(path).await?;
    let data = ron::from_str(&string)?;
    Ok(data)
}
