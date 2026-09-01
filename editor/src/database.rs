mod block;

use std::path::{Path, PathBuf};

use ddo_core::{attribute::Attribute, items::feat::Feat, property::Property, traits::IterValues};
use iced::Task;
use iced_ui::{model::Model, signal::Signal};
use itertools::chain;

use crate::database::block::{DataBlock, DataBlockMsg};

const DATA_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/data/");

#[derive(Debug, Clone)]
pub struct Database {
    feats: DataBlock<Vec<Feat>>,
}

#[derive(Clone, Debug, derive_more::From)]
pub enum DatabaseMsg {
    Feat(DataBlockMsg<Vec<Feat>>),
}

impl Default for Database {
    fn default() -> Self {
        Self {
            feats: DataBlock::new(data_path("feats.ron")),
        }
    }
}

impl Model for Database {
    type Context<'a>
        = ()
    where
        Self: 'a;
    type OutMessage = ();
    type Message = DatabaseMsg;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        (): Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            DatabaseMsg::Feat(msg) => self.feats.empty_update(msg, ()),
        }
    }
}

impl Database {
    pub fn load_all(&self) -> Task<DatabaseMsg> {
        Task::batch([self.feats.load().map(Into::into)])
    }

    pub const fn feats(&self) -> &DataBlock<Vec<Feat>> {
        &self.feats
    }

    fn database_attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        chain!(self
            .feats
            .iter()
            .map(|feat| Attribute::Feat(feat.name().clone())))
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        chain!(Attribute::values(), self.database_attributes())
    }

    pub fn properties(&self) -> impl Iterator<Item = Property> + '_ {
        chain!(
            Property::values(),
            self.database_attributes().map(Property::Attribute)
        )
    }
}

pub fn data_path(file: &str) -> PathBuf {
    Path::new(DATA_ROOT).join(file)
}
