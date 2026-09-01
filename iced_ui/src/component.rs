use iced::Element;

use crate::model::Model;

pub trait Component: Model {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message>;
    fn render_into<'a, M>(&'a self, context: Self::Context<'a>) -> Element<'a, M>
    where
        Self::Message: Into<M>,
        M: 'a,
    {
        self.render(context).map(Into::into)
    }
}
