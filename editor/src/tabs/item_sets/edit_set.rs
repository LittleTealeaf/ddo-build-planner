use builder::equipment::set_bonus::ItemSet;
use iced::{
    theme,
    widget::{button, column, horizontal_space, row, scrollable, text, text_input},
    Application, Command, Element, Length, Renderer,
};
use ui::{HandleMessage, HandleView};

use crate::{modals::bonus_template::ModalBonus, App, Message};

use super::TabSetBonusesMessage;

#[derive(Debug, Clone)]
pub struct ItemSetEditor {
    pub(super) item_set: ItemSet,
    pub(super) index: Option<usize>,
}

impl ItemSetEditor {
    pub const fn new(item_set: ItemSet, index: Option<usize>) -> Self {
        Self { item_set, index }
    }
}

#[derive(Debug, Clone)]
pub enum ItemSetEditorMessage {
    SetName(String),
    CreateTier(i32),
    DeleteTier(i32),
    OpenAddTierBonus(i32),
    OpenEditTierBonus(i32, usize),
    OnAddTierBonus(i32),
    OnEditTierBonus(i32, usize),
    DeleteTierBonus(i32, usize),
}

impl From<ItemSetEditorMessage> for Message {
    fn from(value: ItemSetEditorMessage) -> Self {
        TabSetBonusesMessage::Editing(value).into()
    }
}

impl HandleMessage<ItemSetEditorMessage> for App {
    fn handle_message(
        &mut self,
        message: ItemSetEditorMessage,
    ) -> Command<<Self as Application>::Message> {
        let Some(editor) = &mut self.tab_item_sets.editing else {
            return Command::none();
        };

        match message {
            ItemSetEditorMessage::SetName(string) => {
                editor.item_set.set_name(string);
                Command::none()
            }
            ItemSetEditorMessage::CreateTier(tier) => {
                editor.item_set.bonuses_mut().insert(tier, Vec::new());
                Command::none()
            }
            ItemSetEditorMessage::DeleteTier(tier) => {
                editor.item_set.bonuses_mut().remove(&tier);
                Command::none()
            }
            ItemSetEditorMessage::OpenAddTierBonus(tier) => {
                self.modal_bonus = Some(
                    ModalBonus::new(None)
                        .on_submit(ItemSetEditorMessage::OnAddTierBonus(tier))
                        .title("Add Bonus"),
                );
                Command::none()
            }
            ItemSetEditorMessage::OpenEditTierBonus(tier, index) => {
                let bonus = editor
                    .item_set
                    .bonuses()
                    .get(&tier)
                    .and_then(|tier| tier.get(index));

                self.modal_bonus = Some(
                    ModalBonus::new(bonus)
                        .on_submit(ItemSetEditorMessage::OnEditTierBonus(tier, index))
                        .title("Edit Message"),
                );
                Command::none()
            }
            ItemSetEditorMessage::OnAddTierBonus(tier) => {
                let Some(modal) = self.modal_bonus.as_ref() else {
                    return self.handle_warning("Bonus Modal not open");
                };

                let Some(tier) = editor.item_set.bonuses_mut().get_mut(&tier) else {
                    return self.handle_warning(format!("Tier {tier} does not exist"));
                };

                let Some(bonus) = modal.get_bonus() else {
                    return self.handle_warning("Bonus Modal returns no bonus");
                };

                tier.push(bonus);

                Command::none()
            }
            ItemSetEditorMessage::OnEditTierBonus(tier, index) => {
                let Some(modal) = self.modal_bonus.as_ref() else {
                    return self.handle_warning("Bonus Modal not open");
                };

                let Some(tier) = editor.item_set.bonuses_mut().get_mut(&tier) else {
                    return self.handle_warning(format!("Tier {tier} does not exist"));
                };

                let Some(pointer) = tier.get_mut(index) else {
                    return self.handle_warning(format!("Index {index} does not exist"));
                };

                let Some(bonus) = modal.get_bonus() else {
                    return self.handle_warning("Bonus Modal returns no bonus");
                };

                *pointer = bonus;

                Command::none()
            }
            ItemSetEditorMessage::DeleteTierBonus(tier, index) => {
                let Some(tier) = editor.item_set.bonuses_mut().get_mut(&tier) else {
                    return self.handle_warning(format!("Tier {tier} does not exist"));
                };

                tier.remove(index);

                Command::none()
            }
        }
    }
}

impl HandleView<App> for ItemSetEditor {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        column!(
            row!(
                text("Editing: ").size(30),
                text_input("Item Set Name", self.item_set.name())
                    .on_input(|string| ItemSetEditorMessage::SetName(string).into())
                    .size(25)
            ),
            scrollable(column!(text("hi"))).height(Length::Fill),
            row!(
                horizontal_space(),
                button(text("Cancel").size(20))
                    .on_press(TabSetBonusesMessage::CancelEdit.into())
                    .style(theme::Button::Secondary),
                button(text("Save").size(20))
                    .on_press(TabSetBonusesMessage::SaveEdit.into())
                    .style(theme::Button::Primary)
            )
        )
        .into()
    }
}
