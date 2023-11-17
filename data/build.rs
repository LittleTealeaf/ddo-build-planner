//! Compiles the sourced data into the build file
use std::{env, fs::File, io::Write, path::Path};

use errors::Error;
use serde::Serialize;

fn main() -> Result<(), self::errors::Error> {
    write_artifact(
        "test",
        String::from("This is test data from the build script"),
    )?;

    Ok(())
}

fn write_artifact<T>(name: &str, item: T) -> Result<(), Error>
where
    T: Serialize,
{
    let path = Path::new(&env::var("OUT_DIR")?).join(name);

    let mut file = File::create(path)?;

    file.write_all(ron::to_string(&item)?.as_bytes())?;

    Ok(())
}

mod errors {
    use std::env::VarError;

    #[derive(Debug)]
    pub enum Error {
        Environment(VarError),
        Serialize(ron::Error),
        IO(std::io::Error),
    }

    impl From<VarError> for Error {
        fn from(value: VarError) -> Self {
            Self::Environment(value)
        }
    }

    impl From<ron::error::Error> for Error {
        fn from(value: ron::Error) -> Self {
            Self::Serialize(value)
        }
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::IO(value)
        }
    }
}
