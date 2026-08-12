use iced::Element;

pub trait Render {
    type RenderMessage;
    type ViewContext<'a>
    where
        Self: 'a;
    fn render<'a>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, Self::RenderMessage>;
    fn render_into<'a, M>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, M>
    where
        Self::RenderMessage: Into<M>,
        M: 'a,
    {
        self.render(context).map(Into::into)
    }
}
