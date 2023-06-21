use std::{env, fs::File, path::Path};

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

    let file = File::create(path)?;

    ciborium::into_writer(&item, file)?;

    // file.write_all(ron::to_string(&item)?.as_bytes())?;

    Ok(())
}

mod errors {
    use std::env::VarError;

    #[derive(Debug)]
    pub enum Error {
        Environment(VarError),
        Serialize(ron::Error),
        IO(std::io::Error),
        Value(String),
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

    impl From<ciborium::ser::Error<std::io::Error>> for Error {
        fn from(value: ciborium::ser::Error<std::io::Error>) -> Self {
            match value {
                ciborium::ser::Error::Io(io) => Self::IO(io),
                ciborium::ser::Error::Value(val) => Self::Value(val),
            }
        }
    }
}
