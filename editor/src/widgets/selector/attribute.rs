use builder::attribute::Attribute;
use iced::{Application, Command, Element, Renderer};
use ui::{HandleMessage, HandleView};

use crate::App;

use super::{IntoSelectorMessage, SelectorInternalMessage, SelectorMessage, SelectorWidgetMessage};

#[derive(Debug, Clone)]
pub struct AttributeSelector {
    depth: usize,
    selected: Option<usize>,
    filter: String,
    on_submit: SelectorWidgetMessage,
    on_cancel: SelectorWidgetMessage,
}

impl AttributeSelector {
    pub const fn new(
        depth: usize,
        selected: Option<usize>,
        on_submit: SelectorWidgetMessage,
        on_cancel: SelectorWidgetMessage,
    ) -> Self {
        Self {
            depth,
            selected,
            on_submit,
            on_cancel,
            filter: String::new(),
        }
    }

    pub fn get_attribute<'a>(&self, attributes: &'a [Attribute]) -> Option<&'a Attribute> {
        self.selected.and_then(|index| attributes.get(index))
    }
}

#[derive(Debug, Clone)]
pub enum AttributeSelectorMessage {
    Select(usize),
    Clear,
    Filter(String),
}

impl IntoSelectorMessage for AttributeSelectorMessage {
    fn into_selector_message(self, depth: usize) -> SelectorWidgetMessage {
        SelectorWidgetMessage::Selector(depth, SelectorMessage::Attribute(self))
    }
}

impl<'a> HandleMessage<SelectorInternalMessage<'a>, App> for AttributeSelector {
    fn handle_message(
        &mut self,
        message: SelectorInternalMessage<'a>,
    ) -> Command<<App as Application>::Message> {
        if message.depth != self.depth {
            return Command::none();
        }

        let SelectorMessage::Attribute(attribute_message) = message.content else {
            return Command::none();
        };

        match attribute_message {
            AttributeSelectorMessage::Select(selected) => {
                self.selected = Some(selected);
                Command::none()
            }
            AttributeSelectorMessage::Clear => {
                self.selected = None;
                Command::none()
            }
            AttributeSelectorMessage::Filter(filter) => {
                self.filter = filter;
                Command::none()
            }
        }
    }
}

impl HandleView<App> for AttributeSelector {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        todo!()
        // let attributes = &app.selector.as_ref().expect("Expected Selector").attributes;
        //
        // let filter = self.filter.to_lowercase();
        // let selected = self.selected.unwrap_or(attributes.len());
        //
        // column!(
        //     text_input("Filter...", &self.filter).on_input(|filter| {
        //         AttributeSelectorMessage::Filter(filter).into_message(self.depth)
        //     }),
        //     scrollable(column(
        //         attributes
        //             .iter()
        //             .enumerate()
        //             .map(|(index, attribute)| (index, format!("{attribute}")))
        //             .filter(|(_, str)| matches(&filter, str.to_lowercase().as_ref()))
        //             .map(|(index, attr)| {
        //                 container(
        //                     button(text(attr))
        //                         .on_press(
        //                             AttributeSelectorMessage::Select(index)
        //                                 .into_message(self.depth),
        //                         )
        //                         .style(if selected == index {
        //                             theme::Button::Primary
        //                         } else {
        //                             theme::Button::Text
        //                         })
        //                         .width(Length::Fill),
        //                 )
        //                 .width(Length::Fill)
        //                 .into()
        //             })
        //     ))
        //     .width(Length::Fill)
        //     .height(Length::Fill),
        //     row!(
        //         horizontal_space().width(Length::Fill),
        //         button(text("Cancel"))
        //             .style(theme::Button::Secondary)
        //             .on_press(Message::Selector(self.on_cancel.clone())),
        //         horizontal_space().width(10),
        //         button(text("Submit"))
        //             .style(theme::Button::Primary)
        //             .on_press(Message::Selector(self.on_submit.clone()))
        //     )
        // )
        // .into()
    }
}
