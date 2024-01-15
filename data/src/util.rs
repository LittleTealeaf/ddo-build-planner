#[allow(unused_macros)]
macro_rules! include_data {
    ($file: expr) => {
        ron::from_str(include_str!(concat!(env!("OUT_DIR"), "/", $file)))
    };
}

pub type ParseError = ron::error::SpannedError;
