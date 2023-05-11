#[macro_export]
macro_rules! simple_enum {
    ($enum_name: ident, ($($id: ident $name: expr),*)) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub enum $enum_name {
            $($id),*
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$id => $name),*
                })
            }
        }
    };
}
