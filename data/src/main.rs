//! Loads and saves the data files to update their serialization

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::PathBuf,
};

use anyhow::Result;
use builder::equipment::set_bonus::ItemSet;
use ron::{de::from_reader, ser::to_string_pretty};
use serde::{Deserialize, Serialize};
use utils::ron::pretty_config::compact_pretty_config;

fn main() -> Result<()> {
    let root: PathBuf = PathBuf::from_iter([".", "data", "data"]);
    process_file::<Vec<ItemSet>>(root.join("item_sets.ron"))?;

    Ok(())
}

fn process_file<T>(path: PathBuf) -> Result<()>
where
    for<'de> T: Deserialize<'de> + Serialize,
{
    // Read file
    let file = OpenOptions::new().read(true).open(path.clone())?;
    let reader = BufReader::new(file);
    let data: T = from_reader(reader)?;

    // Write file
    let mut file = File::create(path)?;

    let config = compact_pretty_config();
    let serialized = to_string_pretty(&data, config)?;

    file.write_all(serialized.as_bytes())?;

    Ok(())
}
