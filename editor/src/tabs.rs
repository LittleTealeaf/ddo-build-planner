pub mod home;
pub mod item_sets;
pub mod sandbox;

use core::fmt::{Display, Formatter, Result};

use iced::{widget::Column, Application, Element, Renderer};
use iced_aw::{TabBar, TabLabel};
use ui::HandleView;

use crate::{App, Message};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Home,
    ItemSets,
    Sandbox,
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Home => write!(f, "Home"),
            Self::ItemSets => write!(f, "Item Sets"),
            Self::Sandbox => write!(f, "Sandbox"),
        }
    }
}

impl HandleView<App> for Tab {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'a, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        Column::new()
            .push(
                [Self::Home, Self::ItemSets, Self::Sandbox]
                    .into_iter()
                    .fold(TabBar::new(Message::ChangeTab), |bar, tab| {
                        let label = format!("{tab}");
                        bar.push(tab, TabLabel::Text(label))
                    })
                    .set_active_tab(self),
            )
            .push(match self {
                Self::Home => app.tab_home.handle_view(app),
                Self::ItemSets => app.tab_item_sets.handle_view(app),
                Self::Sandbox => app.tab_sandbox.handle_view(app),
            })
            .into()
    }
}
