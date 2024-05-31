//! Compiles the sourced data into the build file

#![allow(clippy::std_instead_of_core)]
use std::{
    env,
    fs::{read_dir, File, ReadDir},
    io::{BufReader, Write},
    path::Path,
};

use anyhow::Result;
use builder::equipment::set_bonus::ItemSet;
use ron::de::from_reader;
use serde::Serialize;

fn main() -> Result<()> {
    write_artifact(
        "test",
        String::from("This is test data from the build script"),
    )?;

    write_artifact("item_sets", item_sets()?)?;

    Ok(())
}

fn write_artifact<S>(name: &str, item: S) -> Result<()>
where
    S: Serialize,
{
    let path = Path::new(&env::var("OUT_DIR")?).join(name);

    let mut file = File::create(path)?;

    file.write_all(ron::to_string(&item)?.as_bytes())?;

    Ok(())
}

fn get_data_files(dir: &str) -> Result<ReadDir> {
    let path = Path::new(".").join("data").join(dir);
    let path_str = path.to_str().unwrap();

    println!("cargo:rerun-if-changed={path_str}");

    Ok(read_dir(path)?)
}

fn item_sets() -> Result<Vec<ItemSet>> {
    println!("cargo:rerun-if-changed=./data/item_sets.ron");
    let path = Path::new("./data/item_sets.ron");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let items = from_reader(reader)?;
    Ok(items)
}
