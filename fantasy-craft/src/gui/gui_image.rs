use macroquad::prelude::*;
use macroquad::math::UVec2;
use serde::Deserialize;

use crate::{gui::{gui_box::GuiBox, resources::UiResolvedRects}, prelude::{ColorData, ComponentLoader, Context, Transform, UVec2Data, Visible}};

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

pub fn gui_image_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiImage, &Transform, Option<&GuiBox>, Option<&Visible>)>();

    for (entity, (gui_image, transform, gui_box_opt, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible {
            continue;
        }

        if !gui_image.screen_space {
            continue;
        }

        if let Some(spritesheet_name) = &gui_image.texture {
            if let Some(spritesheet) = ctx.asset_server.get_spritesheet(&spritesheet_name) {
                let texture = &spritesheet.texture;
                let source = spritesheet.get_source_rect(gui_image.col_row.x, gui_image.col_row.y);

                let (draw_x, draw_y, dest_size) = 
                    if gui_box_opt.is_some() {
                        // This element has a GuiBox, use the resolved rect
                        // --- MODIFIED ---
                        if let Some((pos, size)) = resolved_rects_map.get(&entity) {
                            (pos.x, pos.y, *size)
                        } else {
                            continue; // Not laid out
                        }
                    } else {
                        // No GuiBox (e.g., simple icon). Use transform data.
                        let dest = vec2(
                            spritesheet.sprite_width * transform.scale.x, 
                            spritesheet.sprite_height * transform.scale.y
                        );
                        (transform.position.x, transform.position.y, dest)
                    };

                let draw_params = DrawTextureParams {
                    dest_size: Some(dest_size),
                    source,
                    ..Default::default()
                };

                draw_texture_ex(texture, draw_x, draw_y, gui_image.tint, draw_params);
            }
        }
    }
}
