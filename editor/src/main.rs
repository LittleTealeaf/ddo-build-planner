//! Application starting point for the Editor Application

use editor::app::Application;
use iced::{Application as _, Settings};

fn main() -> iced::Result {
    Application::run(Settings::default())
}
