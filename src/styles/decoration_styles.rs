use iced::widget::button;

pub struct DecorationClose;
impl button::StyleSheet for DecorationClose {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        default_appearance()
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        hovered_appearance(iced::Color::new(1.0, 0.588, 0.541, 1.0))
    }
}

pub struct DecorationMaximise;
impl button::StyleSheet for DecorationMaximise {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        default_appearance()
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        hovered_appearance(iced::Color::new(1.0, 0.784, 0.596, 1.0))
    }
}

pub struct DecorationMinimise;
impl button::StyleSheet for DecorationMinimise {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        default_appearance()
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        hovered_appearance(iced::Color::new(0.8, 0.886, 0.796, 1.0))
    }
}


fn default_appearance() -> button::Appearance {
    button::Appearance {
        text_color: iced::Color::BLACK,
        background: Some(iced::Background::Color(iced::Color::new(1.0, 1.0, 1.0, 1.0))),
        border: iced::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: 14.0.into(),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.752, 0.764, 0.819, 1.0),
            offset: iced::Vector {
                x: 4.0,
                y: 1.0,
            },
            blur_radius: 3.0,
        },
        shadow_offset: iced::Vector {
            x: 0.0,
            y: 3.0,
        },
    }
}
fn hovered_appearance(color: iced::Color) -> button::Appearance {
    button::Appearance {
        text_color: iced::Color::BLACK,
        background: Some(iced::Background::Color(color)),
        border: iced::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: 14.0.into(),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.752, 0.764, 0.819, 1.0),
            offset: iced::Vector {
                x: 4.0,
                y: 1.0,
            },
            blur_radius: 3.0,
        },
        shadow_offset: iced::Vector {
            x: 0.0,
            y: 3.0,
        },
    }
}