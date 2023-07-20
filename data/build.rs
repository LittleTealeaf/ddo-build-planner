use std::{env, fs::File, io, path::Path};

use serde::Serialize;

fn main() -> Result<(), BuildError> {
    write_artifact(
        "test",
        String::from("This is test data from the build script"),
    )?;

    Ok(())
}

fn write_artifact<T>(name: &str, item: T) -> Result<(), BuildError>
where
    T: Serialize,
{
    let path = Path::new(&env::var("OUT_DIR")?).join(name);

    let file = File::create(path)?;

    ciborium::into_writer(&item, file)?;

    Ok(())
}

#[derive(Debug)]
enum BuildError {
    Environment(env::VarError),
    Ron(ron::Error),
    IO(io::Error),
    Ciborium(ciborium::ser::Error<std::io::Error>),
}

impl From<env::VarError> for BuildError {
    fn from(value: env::VarError) -> Self {
        Self::Environment(value)
    }
}

impl From<ron::Error> for BuildError {
    fn from(value: ron::Error) -> Self {
        Self::Ron(value)
    }
}

impl From<io::Error> for BuildError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<ciborium::ser::Error<std::io::Error>> for BuildError {
    fn from(value: ciborium::ser::Error<std::io::Error>) -> Self {
        Self::Ciborium(value)
    }
}
