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

impl Stylesheet for TextInputStyle {
    fn active(&self) -> Style {
        Style {
            background: match background {
                is_dark_mode => colors::DARK2.into(),
                _ => colors::LIGHT.into(),
            },
            border_color: DARK4,
            border_radius: 0.0,
            border_width: match border_width {
                is_dark_mode => { 0.0 },
                _ => { 1.0 },
            },
            ..Style::default()
        }
    }

fn focused(&self) -> Style {
    Style {
        background: match background {
            is_dark_mode => colors::DARK2.into(),
            _ => colors::LIGHT2.into(),
        },
        border_color: match border_color {
            is_dark_mode => { GRAY },
            _ => { DARK4 },
        },
        border_width: 1.0,
        ..active()
    }
}

}