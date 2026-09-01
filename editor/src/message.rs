use crate::database::DatabaseMsg;

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Database(DatabaseMsg),
}
