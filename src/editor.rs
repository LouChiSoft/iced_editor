use std::fmt::Debug;
use iced::{alignment, Application, Command, Element, Length, Renderer, theme};
use iced::widget::{button, text};

use crate::widgets::TitleBar;
// use crate::widgets::TabLine;

use crate::styles::DecorationClose;
use crate::styles::DecorationMaximise;
use crate::styles::DecorationMinimise;

pub struct Editor {

}


#[derive(Clone, Debug)]
pub enum EditorMessage {
    // Window Messages
    Minimise, ToggleMaximise, Close, Drag,
}

impl Application for Editor {
    type Executor = iced::executor::Default;
    type Message = EditorMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self{}, Command::none())
    }

    fn title(&self) -> String {
        "Iced editor".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            EditorMessage::Minimise => {
                iced::window::minimize(iced::window::Id::MAIN, true)
            }
            EditorMessage::ToggleMaximise => {
                iced::window::toggle_maximize(iced::window::Id::MAIN)
            }
            EditorMessage::Close => {
                iced::window::close(iced::window::Id::MAIN)
            }
            EditorMessage::Drag => {
                iced::window::drag(iced::window::Id::MAIN)
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {

        TitleBar::new(EditorMessage::Drag, EditorMessage::ToggleMaximise)
            .padding(4.0)
            .spacing(4.0)

            .with_minimise_button(
                button(text("_").horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center).size(18.0))
                    .padding(0)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .on_press(EditorMessage::Minimise)
                    .style(theme::Button::Custom(Box::new(DecorationMinimise)))
                    .into())

            .with_maximise_button(
                button(text("□").horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center).size(18.0))
                    .padding(0)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .on_press(EditorMessage::ToggleMaximise)
                    .style(theme::Button::Custom(Box::new(DecorationMaximise)))
                    .into())
            
            .with_close_button(
                button(text("X").horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center).size(18.0))
                    .padding(0)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .on_press(EditorMessage::Close)
                    .style(theme::Button::Custom(Box::new(DecorationClose)))
                    .into())
            
            .into()

    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }
}