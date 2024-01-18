use iced::{widget::column, Application, Element, Renderer};
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

impl HandleView<Editor> for Tab {
    fn handle_view<'a>(
        &'a self,
        app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        column!(
            [(Self::Home, "Home"), (Self::SetBonuses, "Set Bonuses"),]
                .into_iter()
                .fold(TabBar::new(Message::ChangeTab), |bar, (id, label)| {
                    bar.push(id, TabLabel::Text(label.to_string()))
                })
                .set_active_tab(self),
            match &self {
                Self::Home => THome.handle_view(app),
                Self::SetBonuses => app.set_bonuses.handle_view(app),
            }
        )
        .into()
    }
}
