//! Loads the Nerd Fonts

use iced::Font;

/// Nerd Font Icon Bytes
pub const NERD_FONT_BYTES: &[u8] = include_bytes!("../SymbolsNerdFontMono-Regular.ttf");

/// Nerd Font
pub const NERD_FONT: Font = Font::with_name("Symbols Nerd Font Mono");
