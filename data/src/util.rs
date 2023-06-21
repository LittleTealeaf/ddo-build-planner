#[allow(unused_macros)]
macro_rules! include_data {
    ($type: ident, $file: expr) => {
        ciborium::from_reader::<String, _>(include_str!(concat!(env!("OUT_DIR"), "/", "test")).as_bytes())
    };
}

#[allow(dead_code)]
// TODO: Remove allow
pub type ParseError = ciborium::de::Error<std::io::Error>;
