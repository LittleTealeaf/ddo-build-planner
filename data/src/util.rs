#[allow(unused_macros)]
macro_rules! include_data {
    ($type: ident, $file: expr) => {
        ron::from_str::<$type>(include_str!(concat!(env!("OUT_DIR"), "/", $file)))
    };
}
