#![warn(missing_docs)]
#![allow(
    dead_code,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::module_name_repetitions
)]

//! This crate contains large datasets such as items.

macro_rules! include_data {
    ($type: ident, $file: expr) => {
        ciborium::from_reader::<$type, _>(
            include_str!(concat!(env!("OUT_DIR"), "/", $file)).as_bytes(),
        )
    };
}

#[allow(dead_code)]
/// Errror that is returned if the data could not be parsed
pub type ParseError = ciborium::de::Error<std::io::Error>;

#[cfg(feature = "example")]
pub mod example;
