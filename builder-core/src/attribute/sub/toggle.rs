use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Toggle {
    Blocking,
}

impl ToString for Toggle {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Blocking => "Blocking",
        })
    }
}
