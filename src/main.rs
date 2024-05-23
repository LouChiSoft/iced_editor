mod editor;
mod styles;
mod widgets;

use iced::Application;
use iced::Settings;

fn main() -> iced::Result {

    let settings = Settings {
        window: iced::window::Settings {
            position: iced::window::Position::Centered,
            decorations: false,
            ..Default::default()
        },
        ..Default::default()
    };

    editor::Editor::run(settings)
}
