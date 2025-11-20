use macroquad::prelude::*;
use serde::Deserialize;

use crate::{gui::{gui_box::GuiBox, resources::{PreviousMousePosition, UiResolvedRects}}, prelude::{ComponentLoader, Context, Transform, Visible}};

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

pub fn draggable_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let current_mouse_pos = vec2(mouse_x, mouse_y);
    
    // --- MODIFIED ---
    let delta = current_mouse_pos - ctx.resource::<PreviousMousePosition>().0;
    
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);
    let is_down = is_mouse_button_down(MouseButton::Left);

    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&mut GuiDraggable, &mut Transform, &GuiBox, Option<&Visible>)>();

    for (entity, (draggable, transform, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        // --- MODIFIED ---
        let (_resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue; // Not processed by layout system
            };

        if draggable.is_dragging {
            if !is_down {
                draggable.is_dragging = false;
            } else {
                // This is correct: it modifies the Transform directly,
                // which will be used as the base pos next frame.
                transform.position.x += delta.x;
                transform.position.y += delta.y;
            }
        } else {
            // Use the transform's position for hover checking, as it's
            // the most up-to-date position.
            let x = transform.position.x;
            let y = transform.position.y;
            let w = resolved_size.x;
            let h = resolved_size.y;

            let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

            if is_hovered && is_pressed {
                draggable.is_dragging = true;
            }
        }
    }
}
