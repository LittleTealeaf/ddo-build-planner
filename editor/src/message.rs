#[derive(Debug, Clone)]
pub enum Message {
    CustomCrash(String),
    CrashMessage(crate::components::crash::Message),
}

impl From<crate::components::crash::Message> for Message {
    fn from(value: crate::components::crash::Message) -> Self {
        Self::CrashMessage(value)
    }
}