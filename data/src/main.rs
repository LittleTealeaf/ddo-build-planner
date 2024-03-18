//! Loads and saves the data files to update their serialization

use std::{
    fs::{remove_file, File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
};

use builder::equipment::set_bonus::ItemSet;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};

fn main() {
    update_item_sets();
}

fn update_item_sets() {
    let path = Path::new(".")
        .join("data")
        .join("data")
        .join("item_sets.ron");

    let file = OpenOptions::new().read(true).open(path.clone()).unwrap();

    let data: Vec<ItemSet> = from_reader(BufReader::new(file)).unwrap();

    remove_file(path.clone()).unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(
        to_string_pretty(&data, PrettyConfig::new())
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
}
