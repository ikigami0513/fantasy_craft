use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TextDisplay {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub screen_space: bool
}

impl Default for TextDisplay {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 30.0,
            color: BLACK,
            screen_space: true
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuiBox {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub screen_space: bool,
    pub border_radius: f32,
}

impl Default for GuiBox {
    fn default() -> Self {
        Self {
            width: 100.0,
            height: 40.0,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            screen_space: true,
            border_radius: 0.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Idle,
    Hovered,
    Pressed
}

#[derive(Debug, Clone, Copy)]
pub struct GuiButton {
    pub state: ButtonState,
    pub just_clicked: bool,
    pub hovered_color: Color,
    pub pressed_color: Color,
    pub normal_color: Color
}

impl Default for GuiButton {
    fn default() -> Self {
        Self {
            state: ButtonState::Idle,
            just_clicked: false,
            hovered_color: Color::new(0.0, 0.0, 0.0, 1.0),
            pressed_color: Color::new(0.0, 0.0, 0.0, 1.0),
            normal_color: Color::new(0.0, 0.0, 0.0, 1.0)
        }
    }
}
