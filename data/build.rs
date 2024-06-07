//! Compiles the sourced data into the build file

#![allow(clippy::std_instead_of_core)]
use std::{
    env,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use anyhow::Result;
use builder::equipment::set_bonus::ItemSet;
use ron::de::from_reader;
use serde::Serialize;

fn main() -> Result<()> {
    write_artifact("test", "This is Test Data")?;
    write_artifact("item_sets", item_sets()?)?;
    Ok(())
}

fn write_artifact<P, S>(name: P, item: S) -> Result<()>
where
    P: AsRef<Path>,
    S: Serialize,
{
    let path = Path::new(&env::var("OUT_DIR")?).join(name);
    let mut file = File::create(path)?;
    let serialized = ron::to_string(&item)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn item_sets() -> Result<Vec<ItemSet>> {
    println!("cargo:rerun-if-changed=./data/item_sets.ron");
    let path = Path::new("./data/item_sets.ron");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let items = from_reader(reader)?;
    Ok(items)
}
