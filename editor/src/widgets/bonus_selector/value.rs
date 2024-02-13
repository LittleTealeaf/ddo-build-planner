use builder::{attribute::Attribute, bonus::Value};

use super::BonusSelectorTrait;

#[derive(Clone, Debug, Default)]
pub struct ValueSelector {
    child: Option<ChildSelector>,
}

#[derive(Debug, Clone)]
enum ChildSelector {}

#[derive(Clone, Debug)]
pub enum MValueSelector {
    A
}

impl BonusSelectorTrait for ValueSelector {
    type Message = MValueSelector;

    type Output = Value;

    fn new() -> Self {
        todo!()
    }

    fn set_value(self, value: &Self::Output, attributes: &[Attribute]) -> Self {
        todo!()
    }

    fn get_value(&self, attributes: &[Attribute]) -> Option<Self::Output> {
        todo!()
    }

    fn message(
        &mut self,
        message: Self::Message,
        attributes: &[Attribute],
    ) -> iced::Command<crate::Message> {
        todo!()
    }

    fn view<'a, FSubmit, FConvert>(
        &'a self,
        submit: FSubmit,
        convert: FConvert,
        attributes: &[Attribute],
    ) -> iced::Element<'_, crate::Message, iced::Renderer<crate::AppTheme>>
    where
        FSubmit: Fn(Option<Self::Output>) -> crate::Message + 'a + Clone,
        FConvert: Fn(Self::Message) -> crate::Message + 'a + Clone,
    {
        todo!()
    }
}
