use core::fmt::{self, Display};
use std::iter::once;

use builder::{
    attribute::Attribute,
    bonus::{BonusSource, BonusTemplate},
    breakdowns::Breakdowns,
    equipment::set_bonus::ItemSet,
    types::toggle::Toggle,
};
use iced::{
    widget::{button, checkbox, column, container, row, scrollable, text},
    Application, Command, Element, Length, Renderer,
};
use iced_aw::{TabBar, TabLabel};
use itertools::chain;
use ui::{error, font::nf_icon, ExecuteMessage, HandleMessage, HandleView, ToColumn};

use crate::{modals::bonus_template::ModalBonus, App, Message};

#[derive(Debug, Clone)]
pub struct TabSandbox {
    breakdowns: Breakdowns,
    bonuses: Vec<BonusTemplate>,
    toggles: Vec<Toggle>,
    tab: TabSandboxTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabSandboxTab {
    Bonuses,
    Toggles,
    Breakdowns,
}

impl Display for TabSandboxTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bonuses => write!(f, "Bonuses"),
            Self::Toggles => write!(f, "Toggles"),
            Self::Breakdowns => write!(f, "Breakdowns"),
        }
    }
}

impl TabSandbox {
    pub fn new() -> Self {
        Self {
            breakdowns: Breakdowns::new(),
            bonuses: Vec::new(),
            toggles: Vec::new(),
            tab: TabSandboxTab::Bonuses,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TabSandboxMessage {
    NewBreakdowns,
    RefreshItemSets,
    OpenTrackAttributePrompt,
    OnTrackAttribute,
    UntrackAttribute(Attribute),
    AddBonus,
    OnBonusAdded,
    EditBonus(usize),
    OnBonusEdited(usize),
    DeleteBonus(usize),
    UpdateBonuses,
    SetToggle(Toggle, bool),
    RefreshToggles,
    SetTab(TabSandboxTab),
}

impl From<TabSandboxMessage> for Message {
    fn from(value: TabSandboxMessage) -> Self {
        Self::TabSandbox(value)
    }
}

impl HandleMessage<TabSandboxMessage> for App {
    fn handle_message(
        &mut self,
        message: TabSandboxMessage,
    ) -> Command<<Self as Application>::Message> {
        let tab = &mut self.tab_sandbox;

        match message {
            TabSandboxMessage::SetTab(t) => {
                tab.tab = t;
                Command::none()
            }
            TabSandboxMessage::NewBreakdowns => {
                let tracked = tab
                    .breakdowns
                    .tracked_attributes()
                    .cloned()
                    .collect::<Vec<_>>();

                tab.breakdowns = Breakdowns::new();

                for attribute in tracked {
                    tab.breakdowns.track_attribute(attribute);
                }

                Command::batch([
                    Command::message(TabSandboxMessage::RefreshItemSets),
                    Command::message(TabSandboxMessage::UpdateBonuses),
                    Command::message(TabSandboxMessage::RefreshToggles),
                ])
            }
            TabSandboxMessage::RefreshItemSets => {
                let Some(item_sets) = self.data.item_sets.get() else {
                    return self.handle_message(error!("Item sets not loaded"));
                };

                let dynamic_bonuses = item_sets.iter().cloned().map(ItemSet::to_dynamic_bonus);
                tab.breakdowns.import_dynamic_bonuses(dynamic_bonuses);

                Command::none()
            }
            TabSandboxMessage::OpenTrackAttributePrompt => {
                self.modal_attribute = Some(
                    self.select_attribute()
                        .on_submit(TabSandboxMessage::OnTrackAttribute),
                );
                Command::none()
            }
            TabSandboxMessage::OnTrackAttribute => {
                let Some(modal) = &self.modal_attribute else {
                    return self.handle_message(error!("Attribute modal not open"));
                };

                let Some(attribute) = modal.get_attribute() else {
                    return self.handle_message(error!("Attribute Modal has no selection"));
                };

                tab.breakdowns.track_attribute(attribute);

                Command::none()
            }
            TabSandboxMessage::UntrackAttribute(attribute) => {
                let _ = tab.breakdowns.untrack_attribute(&attribute);
                Command::none()
            }
            TabSandboxMessage::AddBonus => {
                self.modal_bonus = Some(
                    ModalBonus::new(None)
                        .title("Add Bonus")
                        .on_submit(TabSandboxMessage::OnBonusAdded),
                );
                Command::none()
            }
            TabSandboxMessage::OnBonusAdded => {
                let Some(modal) = &self.modal_bonus else {
                    return self.handle_message(error!("Bonus Modal is not open"));
                };

                let Some(bonus) = modal.get_bonus() else {
                    return self.handle_message(error!("Modal does not have valid bonus"));
                };

                tab.bonuses.push(bonus);

                self.handle_message(TabSandboxMessage::UpdateBonuses)
            }
            TabSandboxMessage::EditBonus(index) => {
                let Some(bonus) = tab.bonuses.get(index) else {
                    return Command::message(error!("Invalid Bonus Index {index}"));
                };

                self.modal_bonus = Some(
                    ModalBonus::new(Some(bonus))
                        .on_submit(TabSandboxMessage::OnBonusEdited(index))
                        .title("Edit Bonus"),
                );
                Command::none()
            }
            TabSandboxMessage::OnBonusEdited(index) => {
                let Some(pointer) = tab.bonuses.get_mut(index) else {
                    return self.handle_message(error!("Invalid Bonus Index {index}"));
                };

                let Some(modal) = &self.modal_bonus else {
                    return self.handle_message(error!("Bonus Modal is not open"));
                };

                let Some(bonus) = modal.get_bonus() else {
                    return self.handle_message(error!("Modal does not have valid bonus"));
                };

                *pointer = bonus;

                self.handle_message(TabSandboxMessage::UpdateBonuses)
            }
            TabSandboxMessage::DeleteBonus(index) => {
                tab.bonuses.remove(index);

                Command::none()
            }
            TabSandboxMessage::UpdateBonuses => {
                let bonuses = tab
                    .bonuses
                    .iter()
                    .cloned()
                    .map(|bonus| bonus.to_bonus(BonusSource::Debug(0)));

                tab.breakdowns.insert_bonuses(bonuses);

                Command::none()
            }
            TabSandboxMessage::SetToggle(toggle, value) => {
                tab.breakdowns.insert_bonus(toggle.toggle_bonus(value));
                self.handle_message(TabSandboxMessage::RefreshToggles)
            }
            TabSandboxMessage::RefreshToggles => {
                tab.toggles = tab.breakdowns.get_active_toggles().collect();
                Command::none()
            }
        }
    }
}

impl HandleView<App> for TabSandbox {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        column!(
            row!(button(text("Reload")).on_press(TabSandboxMessage::NewBreakdowns.into())),
            [
                TabSandboxTab::Bonuses,
                TabSandboxTab::Toggles,
                TabSandboxTab::Breakdowns
            ]
            .into_iter()
            .fold(
                TabBar::new(|tab| Message::TabSandbox(TabSandboxMessage::SetTab(tab))),
                |bar, tab| {
                    let label = format!("{tab}");
                    bar.push(tab, TabLabel::Text(label))
                }
            )
            .set_active_tab(&self.tab),
            container(match self.tab {
                TabSandboxTab::Bonuses => {
                    Element::from(column!(
                        row!(button("Create").on_press(TabSandboxMessage::AddBonus.into())),
                        scrollable(column(self.bonuses.iter().enumerate().map(
                            |(index, bonus)| {
                                row!(
                                    button(nf_icon(""))
                                        .on_press(TabSandboxMessage::EditBonus(index).into()),
                                    button(nf_icon(""))
                                        .on_press(TabSandboxMessage::DeleteBonus(index).into()),
                                    text(format!(
                                        "{} {} bonus to {} if {}",
                                        bonus.value(),
                                        bonus.bonus_type(),
                                        bonus.attribute(),
                                        bonus.condition().map_or_else(
                                            || "N/A".to_owned(),
                                            |condition| format!("{condition}")
                                        )
                                    ))
                                )
                                .into()
                            }
                        )))
                    ))
                }
                TabSandboxTab::Toggles => Element::from(scrollable(column(
                    self.breakdowns
                        .get_displayed_toggles()
                        .iter()
                        .map(|toggle| {
                            checkbox(format!("{toggle}"), self.toggles.contains(toggle))
                                .on_toggle(|val| TabSandboxMessage::SetToggle(*toggle, val).into())
                                .into()
                        })
                ))),
                TabSandboxTab::Breakdowns => Element::from(column!(
                    row!(button("Track New Attribute")
                        .on_press(TabSandboxMessage::OpenTrackAttributePrompt.into())),
                    scrollable(
                        self.breakdowns
                            .tracked_breakdowns()
                            .map(|(attribute, breakdown)| {
                                row!(
                                    button(nf_icon("󰜺")).on_press(
                                        TabSandboxMessage::UntrackAttribute(attribute.clone())
                                            .into()
                                    ),
                                    column!(
                                        text(attribute),
                                        text(format!("Total: {}", breakdown.value()))
                                    ),
                                    column(
                                        breakdown
                                            .bonuses()
                                            .iter()
                                            .map(|bt| {
                                                row!(
                                                    text(bt.bonus_type()),
                                                    chain!(
                                                        bt.applied().as_ref().map(|entry| text(
                                                            format!(
                                                                "Applied: {} {}",
                                                                entry.value(),
                                                                entry.bonus()
                                                            )
                                                        ),),
                                                        bt.overwritten().iter().map(|entry| text(
                                                            format!(
                                                                "Overwritten: {} {}",
                                                                entry.value(),
                                                                entry.bonus()
                                                            )
                                                        )),
                                                        bt.disabled().iter().map(|entry| text(
                                                            format!(
                                                                "Disabled: {} {}",
                                                                entry.value(),
                                                                entry.bonus()
                                                            )
                                                        ))
                                                    )
                                                    .to_column()
                                                )
                                                .into()
                                            })
                                            .chain(once(
                                                row!(
                                                    text("Stacking"),
                                                    chain!(
                                                        breakdown.stacking().iter().map(|entry| {
                                                            text(format!(
                                                                "{} {}",
                                                                entry.value(),
                                                                entry.bonus()
                                                            ))
                                                        }),
                                                        breakdown.disabled_stacking().iter().map(
                                                            |entry| {
                                                                text(format!(
                                                                    "Disabled: {} {}",
                                                                    entry.value(),
                                                                    entry.bonus()
                                                                ))
                                                            }
                                                        )
                                                    )
                                                    .to_column()
                                                )
                                                .into()
                                            ))
                                    )
                                )
                            })
                            .to_column()
                    )
                )),
            })
            .height(Length::Fill)
        )
        .into()
    }
}
