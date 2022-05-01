mod colors;

use iced::{button, container, rule, text_input, Color};

pub struct MainStyle {
    pub is_dark_mode: bool,
}

impl Stylesheet for MainStyle {
    fn style(&self) -> Style {
        Style {
            background: match background {
                is_dark_mode => colors::DARK.into(),
                _ => colors::LIGHT.into(),
            },
            ..Style::default()
            }
    }
}

pub struct TextStyle {
    pub is_dark_mode: bool,
}

impl Stylesheet for TextStyle {
    fn style(&self) -> Style {
        Style {
            background: match background {
                is_dark_mode => colors::DARK.into(),
                _ => colors::LIGHT.into(),
            },
            ..Style::default()
            }
    }
}

pub struct TextInputStyle {
    pub is_dark_mode: bool,
}

