
macro_rules! include_data {
    ($variable: ident, $file: expr) => {
        let $variable: &str = include!(concat!(env!("OUT_DIR"), "/", $file));

    };
}

pub(crate) use include_data;
