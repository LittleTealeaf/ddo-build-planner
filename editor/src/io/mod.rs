//! Handles input and output of data

use utils::public_modules;

public_modules!(set_bonuses);

#[derive(Debug)]
/// Describes an error caught from reading or saving data
pub enum DataError {
    /// [`std::io::Error`] Errors
    IO(std::io::Error),
    /// [`ron::de::SpannedError`] Errors
    Ron(ron::de::SpannedError),
}

impl From<std::io::Error> for DataError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<ron::de::SpannedError> for DataError {
    fn from(value: ron::de::SpannedError) -> Self {
        Self::Ron(value)
    }
}
