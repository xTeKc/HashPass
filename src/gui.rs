use iced::{button, container, rule, text_input, Color};

//

pub struct AppStyle {
    pub is_dark_mode: bool,
    pub is_running: bool,
    pub opacity: f32,
}

impl container::Stylesheet for AppStyle {
    fn style(&self) -> Style {

    }
}