#[allow(unused_macros)]
macro_rules! include_data {
    ($type: ident, $file: expr) => {
        ron::from_str::<$type>(include_str!(concat!(env!("OUT_DIR"), "/", $file)))
    };
}

#[allow(dead_code)]
// TODO: Remove allow
pub type ParseError = ron::error::SpannedError;
