use core::{
    fmt::{self, Display},
    iter::once,
};

use builder::{
    attribute::Attribute,
    bonus::{BonusSource, BonusTemplate},
    breakdowns::{Breakdowns, DiceStrategy},
    equipment::set_bonus::ItemSet,
    types::{self, toggle::Toggle},
};
use iced::{
    widget::{button, checkbox, column, container, pick_list, row, scrollable, slider, text},
    Application, Command, Element, Length, Renderer,
};
use iced_aw::{TabBar, TabLabel};
use itertools::chain;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use ui::{error, font::nf_icon, warning, ExecuteMessage, HandleMessage, HandleView, ToColumn};

use crate::{modals::bonus_template::ModalBonus, App, Message};

type SliderAttribute = types::slider::Slider;

#[derive(Debug, Clone)]
pub struct TabSandbox {
    breakdowns: Breakdowns,
    bonuses: Vec<BonusTemplate>,
    toggles: Vec<Toggle>,
    /// (Current, Max)
    slider: Vec<(SliderAttribute, f32, f32)>,
    tab: TabSandboxTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabSandboxTab {
    Bonuses,
    Toggles,
    Sliders,
    Breakdowns,
}

impl Display for TabSandboxTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bonuses => write!(f, "Bonuses"),
            Self::Toggles => write!(f, "Toggles"),
            Self::Sliders => write!(f, "Sliders"),
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
            slider: Vec::new(),
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
    SetSider(SliderAttribute, f32),
    RefreshSliders,
    SetDiceStrategy(DiceStrategy),
}

type Msg = TabSandboxMessage;

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
            TabSandboxMessage::SetDiceStrategy(strategy) => {
                tab.breakdowns.set_dice_strategy(strategy);
                Command::none()
            }
            TabSandboxMessage::SetTab(t) => {
                tab.tab = t;
                Command::none()
            }
            TabSandboxMessage::NewBreakdowns => {
                let tracked = tab
                    .breakdowns
                    .breakdowns()
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>();

                tab.breakdowns = Breakdowns::new();

                for attribute in tracked {
                    tab.breakdowns.add_breakdown(attribute);
                }

                let toggles = tab.toggles.iter().map(|toggle| toggle.toggle_bonus(true));
                let sliders = tab.slider.iter().filter_map(|(slider, value, _)| {
                    Some(slider.slider_bonus(Decimal::from_f32(*value)?))
                });

                let bonuses = toggles.chain(sliders);
                tab.breakdowns.insert_bonuses(bonuses);

                Command::batch([
                    Command::message(Msg::RefreshItemSets),
                    Command::message(Msg::UpdateBonuses),
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
                let tracked = tab
                    .breakdowns
                    .breakdowns()
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>();

                self.modal_attribute = Some(
                    self.select_attribute()
                        .on_submit(Msg::OnTrackAttribute)
                        .multiselect(true)
                        .select_all(tracked),
                );
                Command::none()
            }
            TabSandboxMessage::OnTrackAttribute => {
                let Some(modal) = &self.modal_attribute else {
                    return self.handle_message(error!("Attribute modal not open"));
                };

                let attributes = match modal.get_attributes() {
                    Ok(attributes) => attributes,
                    Err(err) => return Command::message(error!("{err}")),
                };

                tab.breakdowns.clear_breakdowns();

                for attribute in attributes {
                    tab.breakdowns.add_breakdown(attribute);
                }

                Command::none()
            }
            TabSandboxMessage::UntrackAttribute(attribute) => {
                if tab.breakdowns.remove_breakdown(&attribute).is_none() {
                    Command::message(warning!("Attribute [{attribute}] was not tracked"))
                } else {
                    Command::none()
                }
            }
            TabSandboxMessage::AddBonus => {
                self.modal_bonus = Some(
                    ModalBonus::new(None)
                        .title("Add Bonus")
                        .on_submit(Msg::OnBonusAdded),
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

                self.handle_message(Msg::UpdateBonuses)
            }
            TabSandboxMessage::EditBonus(index) => {
                let Some(bonus) = tab.bonuses.get(index) else {
                    return Command::message(error!("Invalid Bonus Index {index}"));
                };

                self.modal_bonus = Some(
                    ModalBonus::new(Some(bonus))
                        .on_submit(Msg::OnBonusEdited(index))
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

                self.handle_message(Msg::UpdateBonuses)
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

                self.handle_message(Msg::RefreshSliders)
            }
            TabSandboxMessage::SetToggle(toggle, value) => {
                tab.breakdowns.insert_bonus(toggle.toggle_bonus(value));
                self.handle_message(Msg::RefreshToggles)
            }
            TabSandboxMessage::RefreshToggles => {
                tab.toggles = tab.breakdowns.get_active_toggles().collect();
                Command::none()
            }
            TabSandboxMessage::SetSider(slider, value) => {
                let Some(value) = Decimal::from_f32(value) else {
                    return self.handle_message(error!("Could not parse value {value}"));
                };
                tab.breakdowns.insert_bonus(slider.slider_bonus(value));
                self.handle_message(TabSandboxMessage::RefreshSliders)
            }
            TabSandboxMessage::RefreshSliders => {
                let values = tab.breakdowns.get_active_sliders().collect::<Vec<_>>();

                let result = values
                    .into_iter()
                    .map(|(slider, value)| {
                        Ok((
                            slider,
                            f32::try_from(value)?,
                            f32::try_from(
                                tab.breakdowns
                                    .evaluate_attribute(&Attribute::SliderMax(slider)),
                            )?,
                        ))
                    })
                    .collect::<Result<Vec<_>, rust_decimal::Error>>();

                let values = match result {
                    Ok(values) => values,
                    Err(error) => {
                        return self.handle_message(error!("Error in converting values: {error}"))
                    }
                };

                tab.slider = values.into_iter().collect();

                Command::none()
            }
        }
    }
}

impl HandleView<App> for TabSandbox {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'a, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        column!(
            row!(
                button(text("Reload")).on_press(Msg::NewBreakdowns.into()),
                text("   Dice Strategy: "),
                pick_list(
                    DiceStrategy::VALUES,
                    Some(self.breakdowns.dice_strategy()),
                    |strategy| TabSandboxMessage::SetDiceStrategy(strategy).into()
                )
            ),
            [
                TabSandboxTab::Bonuses,
                TabSandboxTab::Toggles,
                TabSandboxTab::Sliders,
                TabSandboxTab::Breakdowns
            ]
            .into_iter()
            .fold(
                TabBar::new(|tab| Message::TabSandbox(Msg::SetTab(tab))),
                |bar, tab| {
                    let label = format!("{tab}");
                    bar.push(tab, TabLabel::Text(label))
                }
            )
            .set_active_tab(&self.tab),
            container(match self.tab {
                TabSandboxTab::Bonuses => {
                    Element::from(column!(
                        row!(button("Create").on_press(Msg::AddBonus.into())),
                        scrollable(column(self.bonuses.iter().enumerate().map(
                            |(index, bonus)| {
                                row!(
                                    button(nf_icon("")).on_press(Msg::EditBonus(index).into()),
                                    button(nf_icon("")).on_press(Msg::DeleteBonus(index).into()),
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
                                .on_toggle(|val| Msg::SetToggle(*toggle, val).into())
                                .into()
                        })
                ))),
                TabSandboxTab::Sliders => Element::from(scrollable(column(
                    self.slider.iter().map(|(slder, value, max)| {
                        row!(
                            text(format!("{slder}")),
                            slider(0f32..=*max, *value, |value| {
                                Msg::SetSider(*slder, value).into()
                            }),
                            text(format!("{value}"))
                        )
                        .into()
                    })
                ))),
                TabSandboxTab::Breakdowns => Element::from(column!(
                    row!(button("Track New Attribute")
                        .on_press(Msg::OpenTrackAttributePrompt.into())),
                    scrollable(
                        self.breakdowns
                            .breakdowns()
                            .iter()
                            .map(|(attribute, breakdown)| {
                                row!(
                                    button(nf_icon("󰜺"))
                                        .on_press(Msg::UntrackAttribute(attribute.clone()).into()),
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
