use macroquad::prelude::*;
use serde::Deserialize;
use crate::{gui::{gui_box::GuiBox, gui_button::GuiButton, resources::UiResolvedRects}, prelude::{ComponentLoader, Context, Visible}};

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

pub fn checkbox_logic_system(ctx: &mut Context) {
    // This system doesn't use the map, no changes needed.
    let mut query = ctx.world.query::<(&GuiButton, &mut GuiCheckbox, Option<&Visible>)>();

    for (_, (button, checkbox, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }
        
        if button.just_clicked {
            checkbox.is_checked = !checkbox.is_checked;
        }
    }
}

pub fn checkbox_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiCheckbox, &GuiBox, Option<&Visible>)>();

    for (entity, (checkbox, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        if checkbox.is_checked {
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

            let padding = w * 0.2;
            draw_line(x + padding, y + padding, x + w - padding, y + h - padding, 2.0, BLACK);
            draw_line(x + w - padding, y + padding, x + padding, y + h - padding, 2.0, BLACK);
        }
    }
}
