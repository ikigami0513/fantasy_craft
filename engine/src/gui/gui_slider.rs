use macroquad::prelude::*;
use serde::Deserialize;
use crate::{gui::{gui_box::GuiBox, resources::UiResolvedRects}, prelude::{ColorData, ComponentLoader, Context, Visible}};

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

pub fn slider_interaction_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);
    let is_down = is_mouse_button_down(MouseButton::Left);

    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&mut GuiSlider, &GuiBox, Option<&Visible>)>();

    for (entity, (slider, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;
        
        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        if slider.is_dragging_handle {
            if !is_down {
                slider.is_dragging_handle = false;
            } else {
                let relative_x = mouse_x - x;
                let normalized_value = (relative_x / w).clamp(0.0, 1.0);
                slider.value = slider.min + normalized_value * (slider.max - slider.min);
            }
        } else if is_hovered && is_pressed {
            slider.is_dragging_handle = true;
            let relative_x = mouse_x - x;
            let normalized_value = (relative_x / w).clamp(0.0, 1.0);
            slider.value = slider.min + normalized_value * (slider.max - slider.min);
        }
    }
}

pub fn slider_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;
    
    let mut query = ctx.world.query::<(&GuiSlider, &GuiBox, Option<&Visible>)>();

    for (entity, (slider, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        let normalized_value = (slider.value - slider.min) / (slider.max - slider.min).max(f32::EPSILON);
        let handle_width = slider.handle_width;
        
        let handle_x = x + (normalized_value * w) - (handle_width / 2.0);

        draw_rectangle(
            handle_x.clamp(x, x + w - handle_width),
            y,
            handle_width,
            h,
            slider.handle_color
        )
    }
}
