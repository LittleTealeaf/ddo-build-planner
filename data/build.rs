//! Compiles the sourced data into the build file
use std::{
    env,
    fs::{read_dir, File, ReadDir},
    io::Write,
    path::Path,
};

use data::set_bonuses;
use errors::Error;
use serde::Serialize;

fn main() -> Result<(), self::errors::Error> {
    write_artifact(
        "test",
        String::from("This is test data from the build script"),
    )?;

    write_artifact("set_bonuses", set_bonuses()?)?;

    Ok(())
}

fn write_artifact(name: &str, item: impl Serialize) -> Result<(), Error> {
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
    use std::{fs::File, io::BufReader};

    use builder::equipment::set_bonus::SetBonus;
    use ron::de::from_reader;

    use crate::{errors, get_data_files};

    pub fn set_bonuses() -> Result<Vec<SetBonus>, errors::Error> {
        let read_dir = get_data_files("set_bonuses")?;

        let mut bonuses = Vec::new();

        for entry in read_dir {
            let path = entry?.path();

            let file = File::open(path)?;

            let reader = BufReader::new(file);

            let item = from_reader(reader)?;

            bonuses.push(item);
        }

        Ok(bonuses)
    }
}

mod errors {
    use std::env::VarError;

    use ron::error::SpannedError;

    #[derive(Debug)]
    pub enum Error {
        Environment(VarError),
        Serialize(ron::Error),
        Spanned(ron::de::SpannedError),
        IO(std::io::Error),
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

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::IO(value)
        }
    }

    impl From<SpannedError> for Error {
        fn from(value: SpannedError) -> Self {
            Self::Spanned(value)
        }
    }
}
