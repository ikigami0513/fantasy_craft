use hecs::Entity;
use macroquad::prelude::*;
use serde::Deserialize;
use crate::{prelude::{UVec2Data, Vec2Data}, scene::scene_loader::ComponentLoader};

#[derive(Deserialize, Debug, Default)]
pub struct ColorData {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

#[derive(Debug)]
pub struct FontComponent(pub String);

pub struct FontComponentLoader;

impl ComponentLoader for FontComponentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = FontComponent(loader_data);

        ctx.world.insert_one(entity, component).expect("Failed to insert FontComponent");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GuiLayout {
    pub x: GuiDimension,
    pub y: GuiDimension
}

impl Default for GuiLayout {
    fn default() -> Self {
        Self {
            x: GuiDimension::Pixels(0.0),
            y: GuiDimension::Pixels(0.0)
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiLayoutLoaderData {
    #[serde(default)]
    pub x: GuiDimensionLoaderData,
    #[serde(default)]
    pub y: GuiDimensionLoaderData,
}

pub struct GuiLayoutLoader;

impl ComponentLoader for GuiLayoutLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiLayoutLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let parse_dimension = |loader_dim: GuiDimensionLoaderData| -> GuiDimension {
            match loader_dim {
                GuiDimensionLoaderData::Pixels(px) => GuiDimension::Pixels(px),
                GuiDimensionLoaderData::Percent(s) => {
                    let value = s.trim_end_matches('%')
                                 .parse::<f32>()
                                 .unwrap_or(0.0); // 0% par défaut
                    GuiDimension::Percent(value / 100.0) 
                }
            }
        };

        let component = GuiLayout {
            x: parse_dimension(loader_data.x),
            y: parse_dimension(loader_data.y),
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiLayout");
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GuiLocalOffset {
    pub x: GuiDimension,
    pub y: GuiDimension
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiLocalOffsetLoaderData {
    pub x: GuiDimensionLoaderData,
    pub y: GuiDimensionLoaderData
}

pub struct GuiLocalOffsetLoader;

impl ComponentLoader for GuiLocalOffsetLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let loader_data: GuiLocalOffsetLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let parse_dimension = |loader_dim: GuiDimensionLoaderData| -> GuiDimension {
            match loader_dim {
                GuiDimensionLoaderData::Pixels(px) => GuiDimension::Pixels(px),
                GuiDimensionLoaderData::Percent(s) => {
                    let value = s.trim_end_matches('%')
                                 .parse::<f32>()
                                 .unwrap_or(0.0); // 0% par défaut
                    GuiDimension::Percent(value / 100.0) 
                }
            }
        };

        let component = GuiLocalOffset {
            x: parse_dimension(loader_data.x),
            y: parse_dimension(loader_data.y)
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiLocalOffset");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlignmentType {
    Left,
    Center,
    Right
}

impl HorizontalAlignmentType {
    pub fn to_str(&self) -> &'static str {
        match self {
            HorizontalAlignmentType::Left => "left",
            HorizontalAlignmentType::Center => "center",
            HorizontalAlignmentType::Right => "right"
        }
    }

    pub fn from_str(value: &str) -> HorizontalAlignmentType {
        match value {
            "left" => HorizontalAlignmentType::Left,
            "center" => HorizontalAlignmentType::Center,
            "right" => HorizontalAlignmentType::Right,
            _ => HorizontalAlignmentType::Left
        }
    }
}

#[derive(Debug, Clone)]
pub struct HorizontalAlignment(pub HorizontalAlignmentType);

pub struct HorizontalAlignmentLoader;

impl ComponentLoader for HorizontalAlignmentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = HorizontalAlignment(HorizontalAlignmentType::from_str(&loader_data));

        ctx.world.insert_one(entity, component).expect("Failed to insert HorizontalAlignment");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlignmentType {
    Top,
    Center,
    Bottom
}

impl VerticalAlignmentType {
    pub fn to_str(&self) -> &'static str {
        match self {
            VerticalAlignmentType::Top => "top",
            VerticalAlignmentType::Center => "center",
            VerticalAlignmentType::Bottom => "bottom"
        }
    }

    pub fn from_str(value: &str) -> VerticalAlignmentType {
        match value {
            "top" => VerticalAlignmentType::Top,
            "center" => VerticalAlignmentType::Center,
            "bottom" => VerticalAlignmentType::Bottom,
            _ => VerticalAlignmentType::Top
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerticalAlignment(pub VerticalAlignmentType);

pub struct VerticalAlignmentLoader;

impl ComponentLoader for VerticalAlignmentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = VerticalAlignment(VerticalAlignmentType::from_str(&loader_data));

        ctx.world.insert_one(entity, component).expect("Failed to insert VerticalAlignment");
    }
}

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

#[derive(Deserialize, Debug, Default)]
pub struct TextDisplayLoaderData {
    pub text: String,
    pub font_size: f32,
    pub color: ColorData,
    pub screen_space: bool
}

pub struct TextDisplayLoader;

impl ComponentLoader for TextDisplayLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: TextDisplayLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = TextDisplay {
            text: loader_data.text,
            font_size: loader_data.font_size,
            color: Color::new(
                loader_data.color.r,
                loader_data.color.g,
                loader_data.color.b,
                loader_data.color.a
            ),
            screen_space: true
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert TextDisplay");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GuiDimension {
    Pixels(f32),
    Percent(f32)
}

impl GuiDimension {
    pub fn resolve(&self, screen_dimension: f32) -> f32 {
        match self {
            GuiDimension::Pixels(px) => *px,
            GuiDimension::Percent(pct) => (*pct * screen_dimension).round()
        }
    }
}

impl Default for GuiDimension {
    fn default() -> Self {
        GuiDimension::Pixels(100.0)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GuiDimensionLoaderData {
    Pixels(f32),
    Percent(String),
}

impl Default for GuiDimensionLoaderData {
    fn default() -> Self {
        GuiDimensionLoaderData::Pixels(100.0)
    }
}

#[derive(Debug, Clone)]
pub struct GuiBox {
    pub width: GuiDimension,
    pub height: GuiDimension,
    pub color: Color,
    pub screen_space: bool,
    pub border_radius: f32,
}

impl Default for GuiBox {
    fn default() -> Self {
        Self {
            width: GuiDimension::Pixels(100.0),
            height: GuiDimension::Pixels(40.0),
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            screen_space: true,
            border_radius: 0.0
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiBoxLoaderData {
    #[serde(default)]
    pub width: GuiDimensionLoaderData,

    #[serde(default)]
    pub height: GuiDimensionLoaderData,
    
    pub color: ColorData,
    pub screen_space: bool,
    pub border_radius: f32
}

pub struct GuiBoxLoader;

impl ComponentLoader for GuiBoxLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiBoxLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let parse_dimension = |loader_dim: GuiDimensionLoaderData| -> GuiDimension {
            match loader_dim {
                GuiDimensionLoaderData::Pixels(px) => GuiDimension::Pixels(px),
                GuiDimensionLoaderData::Percent(s) => {
                    // Enlève le '%' et parse en f32
                    let value = s.trim_end_matches('%')
                                 .parse::<f32>()
                                 .unwrap_or(100.0); // 100% par défaut en cas d'erreur
                    
                    // Convertit en 0.0-1.0
                    GuiDimension::Percent(value / 100.0) 
                }
            }
        };

        let component = GuiBox {
            width: parse_dimension(loader_data.width),
            height: parse_dimension(loader_data.height),
            color: Color::new(
                loader_data.color.r,
                loader_data.color.g,
                loader_data.color.b,
                loader_data.color.a
            ),
            screen_space: loader_data.screen_space,
            border_radius: loader_data.border_radius
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiBox");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Idle,
    Hovered,
    Pressed
}

impl ButtonState {
    pub fn to_str(&self) -> &'static str {
        match self {
            ButtonState::Idle => "idle",
            ButtonState::Hovered => "hovered",
            ButtonState::Pressed => "pressed"
        }
    }

    pub fn from_str(value: &str) -> ButtonState {
        match value {
            "idle" => ButtonState::Idle,
            "hovered" => ButtonState::Hovered,
            "pressed" => ButtonState::Pressed,
            _ => ButtonState::Idle
        }
    }
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

#[derive(Deserialize, Debug, Default)]
pub struct GuiButtonLoaderData {
    pub state: String,
    pub just_clicked: bool,
    pub hovered_color: ColorData,
    pub pressed_color: ColorData,
    pub normal_color: ColorData
}

pub struct GuiButtonLoader;

impl ComponentLoader for GuiButtonLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiButtonLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiButton {
            state: ButtonState::from_str(loader_data.state.as_str()),
            just_clicked: loader_data.just_clicked,
            hovered_color: Color::new(
                loader_data.hovered_color.r,
                loader_data.hovered_color.g,
                loader_data.hovered_color.b,
                loader_data.hovered_color.a
            ),
            pressed_color: Color::new(
                loader_data.pressed_color.r,
                loader_data.pressed_color.g,
                loader_data.pressed_color.b,
                loader_data.pressed_color.a
            ),
            normal_color: Color::new(
                loader_data.normal_color.r,
                loader_data.normal_color.g,
                loader_data.normal_color.b,
                loader_data.normal_color.a
            )
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiButton");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GuiDraggable {
    pub is_dragging: bool
}

impl Default for GuiDraggable {
    fn default() -> Self {
        Self {
            is_dragging: false
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiDraggableLoaderData {
    pub is_dragging: bool
}

pub struct GuiDraggableLoader;

impl ComponentLoader for GuiDraggableLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiDraggableLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiDraggable {
            is_dragging: loader_data.is_dragging
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiDraggable");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GuiSlider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub is_dragging_handle: bool,
    pub handle_color: Color,
    pub handle_width: f32
}

impl Default for GuiSlider {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            is_dragging_handle: false,
            handle_color: DARKGRAY,
            handle_width: 10.0
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiSliderLoaderData {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub is_dragging_handle: bool,
    pub handle_color: ColorData,
    pub handle_width: f32
}

pub struct GuiSliderLoader;

impl ComponentLoader for GuiSliderLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiSliderLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiSlider {
            value: loader_data.value,
            min: loader_data.min,
            max: loader_data.max,
            is_dragging_handle: loader_data.is_dragging_handle,
            handle_color: Color::new(
                loader_data.handle_color.r,
                loader_data.handle_color.g,
                loader_data.handle_color.b,
                loader_data.handle_color.a
            ),
            handle_width: loader_data.handle_width
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiSliderData");
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GuiCheckbox {
    pub is_checked: bool
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiCheckboxLoaderData {
    pub is_checked: bool
}

pub struct GuiCheckboxLoader;

impl ComponentLoader for GuiCheckboxLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiCheckboxLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiCheckbox {
            is_checked: loader_data.is_checked
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiCheckbox");
    }
}

#[derive(Debug, Clone)]
pub struct GuiInputField {
    pub text: String,
    pub is_focused: bool,
    pub caret_blink_timer: f32,
    pub caret_visible: bool,
    pub max_chars: Option<usize>,
    pub font_size: f32,
    pub color: Color,
    pub backspace_repeat_timer: f32,
    pub padding: Vec2,
    pub caret_position: usize,
    pub scroll_offset: f32,
    pub left_key_repeat_timer: f32,
    pub right_key_repeat_timer: f32
}

impl Default for GuiInputField {
    fn default() -> Self {
        Self {
            text: String::new(),
            is_focused: false,
            caret_blink_timer: 0.0,
            caret_visible: true,
            max_chars: None,
            font_size: 30.0,
            color: BLACK,
            backspace_repeat_timer: 0.0,
            padding: vec2(0.0, 0.0),
            caret_position: 0,
            scroll_offset: 0.0,
            left_key_repeat_timer: 0.0,
            right_key_repeat_timer: 0.0
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiInputFieldLoaderData {
    pub text: String,
    pub is_focused: bool,
    pub caret_blink_timer: f32,
    pub caret_visible: bool,
    pub max_chars: Option<usize>,
    pub font_size: f32,
    pub color: ColorData,
    pub backspace_repeat_timer: f32,
    pub padding: Vec2Data,
    pub caret_position: usize,
    pub scroll_offset: f32,
    pub left_key_repeat_timer: f32,
    pub right_key_repeat_timer: f32
}

pub struct GuiInputFieldLoader;

impl ComponentLoader for GuiInputFieldLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiInputFieldLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiInputField {
            text: loader_data.text,
            is_focused: loader_data.is_focused,
            caret_blink_timer: loader_data.caret_blink_timer,
            caret_visible: loader_data.caret_visible,
            max_chars: loader_data.max_chars,
            font_size: loader_data.font_size,
            color: Color::new(
                loader_data.color.r,
                loader_data.color.g,
                loader_data.color.b,
                loader_data.color.a
            ),
            backspace_repeat_timer: loader_data.backspace_repeat_timer,
            padding: vec2(loader_data.padding.x, loader_data.padding.y),
            caret_position: loader_data.caret_position,
            scroll_offset: loader_data.scroll_offset,
            left_key_repeat_timer: loader_data.left_key_repeat_timer,
            right_key_repeat_timer: loader_data.right_key_repeat_timer
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiInputField");
    }
}

#[derive(Debug, Clone)]
pub struct GuiImage {
    pub texture: Option<String>,
    pub col_row: UVec2,
    pub tint: Color,
    pub screen_space: bool
}

impl Default for GuiImage {
    fn default() -> Self {
        Self {
            texture: None,
            col_row: uvec2(0, 0),
            tint: WHITE,
            screen_space: true
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiImageLoaderData {
    pub texture: Option<String>,
    pub col_row: UVec2Data,
    pub tint: ColorData,
    pub screen_space: bool
}

pub struct GuiImageLoader;

impl ComponentLoader for GuiImageLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiImageLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiImage {
            texture: loader_data.texture,
            col_row: uvec2(loader_data.col_row.x, loader_data.col_row.y),
            tint: Color::new(
                loader_data.tint.r,
                loader_data.tint.g,
                loader_data.tint.b,
                loader_data.tint.a
            ),
            screen_space: true
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiImage");
    }
}
