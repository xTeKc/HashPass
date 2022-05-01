#[path = "colors.rs"]
mod colors;

use iced::{button, container, rule, text_input, Color};

pub struct MainStyle {
    pub is_dark_mode: bool,
}

impl container::StyleSheet for MainStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: match background {
                is_dark_mode => colors::DARK.into(),
                _ => colors::LIGHT.into(),
            },
            ..container::Style::default()
            }
    }
}

pub struct TextStyle {
    pub is_dark_mode: bool,
}

impl container::StyleSheet for TextStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: match background {
                is_dark_mode => colors::DARK.into(),
                _ => colors::LIGHT.into(),
            },
            ..container::Style::default()
            }
    }
}

pub struct TextInputStyle {
    pub is_dark_mode: bool,
}

impl text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: match background {
                is_dark_mode => colors::DARK2.into(),
                _ => colors::LIGHT.into(),
            },
            border_color: colors::DARK4,
            border_radius: 0.0,
            border_width: match border_width {
                is_dark_mode => { 0.0 },
                _ => { 1.0 },
            },
            ..text_input::Style::default()
        }
    }

fn focused(&self) -> text_input::Style {
    text_input::Style {
        background: match background {
            is_dark_mode => colors::DARK2.into(),
            _ => colors::LIGHT2.into(),
        },
        border_color: match border_color {
            is_dark_mode => { colors::GRAY },
            _ => { colors::DARK4 },
        },
        border_width: 1.0,
        ..self.active()
    }
}

fn placeholder_color(&self) -> Color {
    colors::GRAY
}

fn value_color(&self) -> Color {
    match self {
        is_dark_mode => colors::LIGHT_TEXT,
        _ => colors::DARK_TEXT,
    }
}

fn selection_color(&self) -> Color {
    match self {
        is_dark_mode => colors::GRAY,
        _ => colors::LIGHT4,
    }
}

}

pub struct RuleStyle;

impl rule::StyleSheet for RuleStyle {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: colors::GRAY,
            width: 1,
            radius: 0.0,
            fill_mode: rule::FillMode::Padded(8),
        }
    }
}

pub struct IndexStyle {
    pub is_dark_mode: bool,
}

impl container::StyleSheet for IndexStyle {
    fn style(&self) -> container::Style {
        let mut c = colors::PRIMARY;
        match self {
            is_dark_mode => c.a = 0.60,
            _ => c.a = 0.0,
        }
        container::Style {
            background: c.into(),
            border_width: 0.0,
            ..container::Style::default()
        }
    }
}

pub struct TooltipStyle;

impl container::StyleSheet for TooltipStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: colors::LIGHT_TEXT.into(),
            background: colors::DARK3.into(),
            ..container::Style::default()
        }
    }
}

pub struct ButtonStyle {
    pub is_dark_mode: bool,
    pub foreground: Option<Color>,
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: match background {
                is_dark_mode => colors::DARK2.into(),
                _ => colors::LIGHT.into(),
            },
            border_color: colors::DARK3,
            border_radius: 2.0,
            border_width: match border_width {
                is_dark_mode => { 0.0 },
                _ => { 1.0 },
            },
            text_color: match foreground.unwrap_or() {
                is_dark_mode => colors::LIGHT_TEXT,
                _ => colors::DARK_TEXT,
            },
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: match background {
                is_dark_mode => colors::DARK3.into(),
                _ => colors::LIGHT2.into(),
            },
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: match background {
                is_dark_mode => colors::DARK4.into(),
                _ => colors::LIGHT3.into(),
            },
            ..self.hovered()
        }
    }

}