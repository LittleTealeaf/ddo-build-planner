//! Loads and saves the data files to update their serialization

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
};

use anyhow::Result;
use builder::equipment::set_bonus::ItemSet;
use ron::{de::from_reader, ser::to_string_pretty};
use utils::ron::pretty_config::compact_pretty_config;

fn main() -> Result<()> {
    item_sets()?;
    Ok(())
}

fn item_sets() -> Result<()> {
    let path = Path::new(".")
        .join("data")
        .join("data")
        .join("item_sets.ron");

    let file = OpenOptions::new().read(true).open(path.clone())?;
    let data: Vec<ItemSet> = from_reader(BufReader::new(file))?;

    let mut file = File::create(path)?;

    file.write_all(to_string_pretty(&data, compact_pretty_config())?.as_bytes())?;

    Ok(())
}
