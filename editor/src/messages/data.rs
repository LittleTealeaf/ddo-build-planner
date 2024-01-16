use builder::equipment::set_bonus::SetBonus;

use super::HandleMessage;
/// Messages for loading / saving data
#[derive(Debug, Clone)]
pub enum DataMessage {
    /// Set Bonuses
    SetBonuses(DataIOMessage<Vec<SetBonus>>),
}

impl HandleMessage for DataMessage {
    fn handle(&self, _app: &mut crate::app::Application) -> iced::Command<super::Message> {
        match self {
            Self::SetBonuses(message) => match message {
                DataIOMessage::StartLoad => todo!(),
                DataIOMessage::FinishLoad(_) => todo!(),
                DataIOMessage::StartSave => todo!(),
                DataIOMessage::FinishSave => todo!(),
            },
        }
    }
}

/// Handles generic messages for loading / saving data
#[derive(Debug, Clone)]
pub enum DataIOMessage<T> {
    /// Indicates that loading should start for a specific data point
    StartLoad,
    /// Indicates that the data has finished loading, returning that data
    FinishLoad(T),
    /// Indicates that saving should start
    StartSave,
    /// Indicates that saving has finished
    FinishSave,
}
