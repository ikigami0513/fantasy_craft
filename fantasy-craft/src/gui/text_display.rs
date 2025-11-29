use macroquad::prelude::*;
use serde::Deserialize;

use crate::{gui::{alignment::{HorizontalAlignment, HorizontalAlignmentType, VerticalAlignment, VerticalAlignmentType}, font_component::FontComponent}, prelude::{ColorData, ComponentLoader, Context, Transform, Visible}};

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

pub fn text_render_system(ctx: &mut Context) {
    // This system correctly uses Transform.position, which is set by the
    // gui_resolve_layout_system. No changes are needed.
    for (_, (text_display, transform, visibility, font_opt, h_align, v_align)) in ctx.world.query::<(&TextDisplay, &Transform, Option<&Visible>, Option<&FontComponent>, Option<&HorizontalAlignment>, Option<&VerticalAlignment>)>().iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible || !text_display.screen_space {
            continue;
        }

        let font = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));
        
        let text_size = measure_text(&text_display.text, font, text_display.font_size as u16, 1.0);

        // --- Alignment Logic (Correct) ---
        let mut draw_x = transform.position.x;
        if let Some(h_align) = h_align {
            match h_align.0 {
                HorizontalAlignmentType::Left => { /* Default */ }
                HorizontalAlignmentType::Center => draw_x = transform.position.x - text_size.width / 2.0,
                HorizontalAlignmentType::Right => draw_x = transform.position.x - text_size.width,
            }
        }
        
        let mut baseline_y = transform.position.y + text_size.offset_y; 
        
        if let Some(v_align) = v_align {
            match v_align.0 {
                VerticalAlignmentType::Top => { /* Default */ }
                VerticalAlignmentType::Center => baseline_y = transform.position.y - (text_size.height / 2.0) + text_size.offset_y,
                VerticalAlignmentType::Bottom => baseline_y = transform.position.y - text_size.height + text_size.offset_y,
            }
        }
        // --- End Alignment Logic ---

        if let Some(font) = font {
            draw_text_ex(
                &text_display.text,
                draw_x.round(),
                baseline_y.round(),
                TextParams {
                    font: Some(font),
                    font_size: text_display.font_size as u16,
                    color: text_display.color,
                    ..Default::default()
                }
            );
        }
        else {
            draw_text(
                &text_display.text,
                draw_x.round(),
                baseline_y.round(),
                text_display.font_size,
                text_display.color
            );
        }
    }
}
