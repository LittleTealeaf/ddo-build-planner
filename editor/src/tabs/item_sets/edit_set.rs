use builder::{equipment::set_bonus::ItemSet, types::item_type};
use iced::{Application, Command, Element, Renderer};
use ui::{HandleMessage, HandleView};

use crate::{modals::bonus_template::ModalBonus, App, Message};

use super::TabSetBonusesMessage;

#[derive(Debug, Clone)]
pub struct ItemSetEditor {
    item_set: ItemSet,
}

impl ItemSetEditor {
    pub const fn new(item_set: ItemSet) -> Self {
        Self { item_set }
    }
}

#[derive(Debug, Clone)]
pub enum ItemSetEditorMessage {
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
                let bonus = self.modal_bonus.as_ref().and_then(ModalBonus::get_bonus);

                if let Some(bonus) = bonus {
                    if let Some(tier) = editor.item_set.bonuses_mut().get_mut(&tier) {
                        tier.push(bonus);
                    }
                }

                Command::none()
            }
            ItemSetEditorMessage::OnEditTierBonus(tier, index) => {
                let Some(bonus) = self.modal_bonus.as_ref().and_then(ModalBonus::get_bonus) else {
                    return Command::none();
                };

                let Some(tier) = editor.item_set.bonuses_mut().get_mut(&tier) else {
                    return Command::none();
                };

                let Some(pointer) = tier.get_mut(index) else {
                    return Command::none();
                };

                *pointer = bonus;

                Command::none()
            }
            ItemSetEditorMessage::DeleteTierBonus(tier, usize) => {
                let Some(tier) = editor.item_set.bonuses_mut().get_mut(&tier) else {
                    return Command::none();
                };

                tier.remove(usize);

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
        todo!()
    }
}
