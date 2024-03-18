//! Loads and saves the data files to update their serialization

use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    path::Path,
};

use builder::equipment::set_bonus::ItemSet;
use ron::{
    de::from_reader,
    ser::{to_writer_pretty, PrettyConfig},
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

    let file = OpenOptions::new().write(true).open(path).unwrap();

    let writer = BufWriter::new(file);
    to_writer_pretty(writer, &data, PrettyConfig::new()).unwrap();
}
