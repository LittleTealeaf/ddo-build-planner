use core::fmt::Debug;

use builder::attribute::Attribute;
use iced::{Command, Element, Renderer};

use crate::{AppTheme, Message};

pub mod attribute;
pub mod condition;
pub mod value;

pub struct BonusSelector<S>
where
    S: BonusSelectorTrait,
{
    selector: S,
    attributes: Vec<Attribute>,
}

impl<S> BonusSelector<S>
where
    S: BonusSelectorTrait,
{
    fn new<I>(attributes: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        let attributes = attributes.into_iter().collect();
        Self {
            selector: S::new(),
            attributes,
        }
    }

    fn set_value(self, value: &S::Output) -> Self {
        Self {
            selector: self.selector.set_value(value, &self.attributes),
            ..self
        }
    }

    fn set_value_maybe(self, value: Option<&S::Output>) -> Self {
        Self {
            selector: self.selector.set_value_maybe(value, &self.attributes),
            ..self
        }
    }

    fn get_value(&self) -> Option<S::Output> {
        self.selector.get_value(&self.attributes)
    }

    fn message(&mut self, message: S::Message) -> Command<Message> {
        self.selector.message(message, &self.attributes)
    }

    fn view<'a, FSubmit, FConvert>(
        &'a self,
        submit: FSubmit,
        convert: FConvert,
    ) -> Element<'_, Message, Renderer<AppTheme>>
    where
        FSubmit: Fn(Option<S::Output>) -> Message + 'a + Clone,
        FConvert: Fn(S::Message) -> Message + 'a + Clone,
    {
        self.selector.view(submit, convert, &self.attributes)
    }
}

pub trait BonusSelectorTrait: Sized {
    type Message: Clone + Debug;
    type Output: Clone;

    fn new() -> Self;

    fn set_value_maybe(self, value: Option<&Self::Output>, attributes: &[Attribute]) -> Self {
        match value {
            Some(value) => self.set_value(value, attributes),
            None => self,
        }
    }

    fn set_value(self, value: &Self::Output, attributes: &[Attribute]) -> Self;

    fn get_value(&self, attributes: &[Attribute]) -> Option<Self::Output>;

    fn message(&mut self, message: Self::Message, attributes: &[Attribute]) -> Command<Message>;

    fn view<'a, FSubmit, FConvert>(
        &'a self,
        submit: FSubmit,
        convert: FConvert,
        attributes: &[Attribute],
    ) -> Element<'_, Message, Renderer<AppTheme>>
    where
        FSubmit: Fn(Option<Self::Output>) -> Message + 'a + Clone,
        FConvert: Fn(Self::Message) -> Message + 'a + Clone;
}
