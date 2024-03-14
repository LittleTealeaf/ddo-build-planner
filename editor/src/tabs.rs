pub mod home;
pub mod set_bonuses;

use core::fmt::{Display, Formatter, Result};

use iced::{widget::Column, Application, Element, Renderer};
use iced_aw::{TabBar, TabLabel};
use ui::HandleView;

use crate::{Editor, Message};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Home,
    SetBonuses,
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Home => write!(f, "Home"),
            Self::SetBonuses => write!(f, "Set Bonuses"),
        }
    }
}

impl HandleView<Editor> for Tab {
    fn handle_view<'a>(
        &'a self,
        app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, <Editor as Application>::Theme, Renderer>
    {
        Column::new()
            .push(
                [Self::Home, Self::SetBonuses]
                    .into_iter()
                    .fold(TabBar::new(Message::ChangeTab), |bar, tab| {
                        let label = format!("{tab}");
                        bar.push(tab, TabLabel::Text(label))
                    })
                    .set_active_tab(self),
            )
            .push(match self {
                Self::Home => app.tab_home.handle_view(app),
                Self::SetBonuses => todo!(),
            })
            .into()
    }
}
