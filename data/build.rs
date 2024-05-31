//! Compiles the sourced data into the build file

#![allow(clippy::std_instead_of_core)]
use std::{
    env,
    fs::{read_dir, File, ReadDir},
    io::Write,
    path::Path,
};

use data::item_sets;
use errors::Error;
use serde::Serialize;

fn main() -> Result<(), self::errors::Error> {
    write_artifact(
        "test",
        String::from("This is test data from the build script"),
    )?;

    write_artifact("item_sets", item_sets()?)?;

    Ok(())
}

fn write_artifact<S>(name: &str, item: S) -> Result<(), Error>
where
    S: Serialize,
{
    let path = Path::new(&env::var("OUT_DIR")?).join(name);

    let mut file = File::create(path)?;

    file.write_all(ron::to_string(&item)?.as_bytes())?;

    Ok(())
}

fn get_data_files(dir: &str) -> Result<ReadDir, Error> {
    let path = Path::new(".").join("data").join(dir);
    let path_str = path.to_str().unwrap();

    println!("cargo:rerun-if-changed={path_str}");

    Ok(read_dir(path)?)
}

mod data {
    use std::{fs::File, io::BufReader, path::Path};

    use builder::equipment::set_bonus::ItemSet;
    use ron::de::from_reader;

    use crate::errors;

    pub fn item_sets() -> Result<Vec<ItemSet>, errors::Error> {
        println!("cargo:rerun-if-changed=./data/item_sets.ron");
        let path = Path::new("./data/item_sets.ron");
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let items = from_reader(reader)?;
        Ok(items)
    }
}

mod errors {
    use std::{env::VarError, io};

    use ron::de::SpannedError;

    #[derive(Debug)]
    pub enum Error {
        Environment(VarError),
        Serialize(ron::Error),
        Spanned(SpannedError),
        IO(io::Error),
    }

    impl From<VarError> for Error {
        fn from(value: VarError) -> Self {
            Self::Environment(value)
        }
    }

    impl From<ron::Error> for Error {
        fn from(value: ron::Error) -> Self {
            Self::Serialize(value)
        }
    }

    impl From<io::Error> for Error {
        fn from(value: io::Error) -> Self {
            Self::IO(value)
        }
    }

    impl From<SpannedError> for Error {
        fn from(value: SpannedError) -> Self {
            Self::Spanned(value)
        }
    }
}
