mod colors;

use iced::{button, container, rule, text_input, Color};

pub struct MainStyle {
    pub is_dark_mode: bool,
}

impl StyleSheet for MainStyle {
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

impl StyleSheet for TextStyle {
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

impl StyleSheet for TextInputStyle {
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

fn placeholder_color(&self) -> Color {
    GRAY
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

impl StyleSheet for RuleStyle {
    fn style(&self) -> Style {
        Style {
            color: colors::GRAY,
            width: 1,
            radius: 0.0,
            fill_mode: FillMode::Padded(8),
        }
    }
}

pub struct IndexStyle {
    pub is_dark_mode: bool,
}

impl StyleSheet for IndexStyle {
    fn style(&self) -> Style {
        let mut c = colors::PRIMARY;
        match self {
            is_dark_mode => c.a = 0.60,
            _ => c.a = 0.0,
        }
        Style {
            background: c.into(),
            border_width: 0.0,
            ..Style::default()
        }
    }
}

pub struct TooltipStyle;

impl StyleSheet for TooltipStyle {
    fn style(&self) -> Style {
        Style {
            text_color: colors::LIGHT_TEXT.into(),
            background: colors::DARK3.into(),
            ..Style::default()
        }
    }
}