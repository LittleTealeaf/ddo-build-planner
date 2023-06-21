use std::{env, fs::File, io::Write, path::Path};

use builder::{
    attribute::{flags::Flag, types::Ability, Attribute},
    bonus::{Bonus, BonusSource, BonusType, Condition},
};
use errors::Error;
use serde::Serialize;

fn main() -> Result<(), self::errors::Error> {
    let test = Bonus::new(
        Attribute::Ability(Ability::Charisma),
        BonusType::Quality,
        4f32.into(),
        BonusSource::Base,
        Some(Condition::Has(Attribute::Flag(Flag::Race(
            builder::race::Race::Drow,
        )))),
    );

    write_artifact("test.ron", test)?;

    Ok(())
}

fn write_artifact<T>(name: &str, item: T) -> Result<(), Error>
where
    T: Serialize,
{
    let path = Path::new(&env::var("OUT_DIR")?).join(name);

    let mut file = File::create(path)?;

    let serialized = format!("\"{}\"", ron::to_string(&item)?);

    file.write_all(&serialized.as_bytes())?;

    Ok(())
}

mod errors {
    use std::env::VarError;

    #[derive(Debug)]
    pub enum Error {
        VarError(VarError),
        SerializeError(ron::Error),
        IOError(std::io::Error),
    }

    impl From<VarError> for Error {
        fn from(value: VarError) -> Self {
            Self::VarError(value)
        }
    }

    impl From<ron::error::Error> for Error {
        fn from(value: ron::Error) -> Self {
            Self::SerializeError(value)
        }
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::IOError(value)
        }
    }
}
