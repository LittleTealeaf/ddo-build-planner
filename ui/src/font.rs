//! Loads the Nerd Fonts

use core::convert::From;
use std::borrow::Cow;

use iced::{
    advanced::text,
    widget::{text::StyleSheet, Text},
    Font,
};

/// Nerd Font Icon Bytes
pub const NERD_FONT_BYTES: &[u8] = include_bytes!("../SymbolsNerdFontMono-Regular.ttf");

/// Nerd Font
pub const NERD_FONT: Font = Font::with_name("Symbols Nerd Font Mono");

/// Creates a [`Text`] object with the given text and [`NERD_FONT`] already loaded
pub fn nf_icon<'a, T, R, S>(text: S) -> Text<'a, T, R>
where
    T: StyleSheet,
    R: text::Renderer,
    S: Into<Cow<'a, str>>,
    <R as text::Renderer>::Font: From<iced::Font>,
{
    Text::new(text).font(NERD_FONT)
}
