use iced::widget::column;
use iced_aw::{TabBar, TabLabel};
use ui::HandleView;
use utils::public_modules;

use crate::{Editor, Message};

public_modules!(set_bonuses, home);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Home,
    SetBonuses,
}

impl HandleView<Tab> for Editor {
    fn handle_view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column!(
            [(Tab::Home, "Home"), (Tab::SetBonuses, "Set Bonuses"),]
                .into_iter()
                .fold(TabBar::new(Message::ChangeTab), |bar, (id, label)| {
                    bar.push(id, TabLabel::Text(label.to_string()))
                })
                .set_active_tab(&self.tab),
            match &self.tab {
                Tab::Home => HandleView::<THome>::handle_view(self),
                Tab::SetBonuses => HandleView::<TSetBonuses>::handle_view(self),
            }
        )
        .into()
    }
}
