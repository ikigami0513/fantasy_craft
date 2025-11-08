use macroquad::prelude::*;
use crate::prelude::{Context, TileMapLayerComponent};
use crate::graphics::tiled_map::components::TileMapComponent;

pub fn tilemap_render_system(ctx: &mut Context) {
    for (_, tileset_comp) in ctx.world.query::<&TileMapComponent>().iter() {
        if let Some(rendered_map) = ctx.asset_server.get_renderer_map(&tileset_comp.0) {
            draw_texture_ex(
                &rendered_map.texture.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rendered_map.width, rendered_map.height)),
                    ..Default::default()
                }
            );
        }
    }
}

pub fn tilemap_layer_render_system(ctx: &mut Context) {
    for (_, layer_comp) in ctx.world.query::<&TileMapLayerComponent>().iter() {
        if let Some(renderer_layer) = ctx.asset_server.get_renderer_layer(
            &layer_comp.tilemap_name,
            &layer_comp.layer_name
        ) {
            let (width, height) = 
                if let Some(map) = ctx.asset_server.get_map(&layer_comp.tilemap_name) {
                    (
                        map.width as f32 * map.tile_width as f32,
                        map.height as f32 * map.tile_height as f32
                    )
                }
                else {
                    (renderer_layer.texture.width(), renderer_layer.texture.height())
                };
            
            draw_texture_ex(
                &renderer_layer.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(width, height)),
                    ..Default::default()
                }
            );
        }
    }
}
