use editor::EditorApp;
use iced_ui::app::App;

/// # Errors
pub fn main() -> iced::Result {
    EditorApp::application().run()
}
