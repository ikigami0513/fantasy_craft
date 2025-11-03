use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use crate::graphics::sprites::Spritesheet;

pub struct Tileset {
    pub first_gid: u32,
    pub spritesheet: Arc<Spritesheet>,
    pub columns: u32,
    pub tile_width: f32,
    pub tile_height: f32
}

pub struct RenderedTileMap {
    pub texture: RenderTarget,
    pub width: f32,
    pub height: f32
}

pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_layers: HashMap<String, Vec<u32>>,
    pub tilesets: Vec<Tileset>
}

impl TileMap {
    pub fn get_tileset_for_gid(&self, gid: u32) -> Option<&Tileset> {
        self.tilesets.iter()
            .rev()
            .find(|ts| gid >= ts.first_gid)
    }

    pub fn get_tile_coords(&self, gid: u32) -> Option<(u32, u32, &Tileset)> {
        if gid == 0 {
            return None
        }

        if let Some(tileset) = self.get_tileset_for_gid(gid) {
            let local_id = gid - tileset.first_gid;

            let col = local_id % tileset.columns;
            let row = local_id / tileset.columns;

            Some((col, row, tileset))
        }
        else {
            None
        }
    }

    pub fn to_render_tilemap(&self) -> RenderedTileMap {
        let tile_w = self.tile_width as f32;
        let tile_h = self.tile_height as f32;

        let width = self.width as f32 * tile_w;
        let height = self.height as f32 * tile_h;

        let render_target = render_target(width as u32, height as u32);
        
        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            target: vec2(width / 2.0, height / 2.0),
            zoom: vec2(2.0 / width, -2.0 / height),
            ..Default::default()
        });

        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        for (_, data) in self.tile_layers.iter() {
            for y in 0..self.height {
                for x in 0..self.width {
                    let index = (y * self.width + x) as usize;
                    let gid = data.get(index).copied().unwrap_or(0);

                    if let Some((col, row, tileset)) = self.get_tile_coords(gid) {
                        let draw_x = x as f32 * tile_w + tile_w / 2.0;
                        let draw_y = y as f32 * tile_h + tile_h / 2.0;
                        tileset.spritesheet.draw_sprite(col, row, draw_x, draw_y, vec2(1.0, 1.0), false);
                    }
                }
            }
        }

        set_default_camera();

        RenderedTileMap {
            texture: render_target,
            width,
            height
        }
    }

    pub fn render_all_layers(&self) -> HashMap<String, RenderTarget> {
        let mut renderer_layers = HashMap::new();

        let tile_w = self.tile_width as f32;
        let tile_h = self.tile_height as f32;
        let width = self.width as f32 * tile_w;
        let height = self.height as f32 * tile_h;

        for (layer_name, data) in self.tile_layers.iter() {
            let render_target = render_target(width as u32, height as u32);
            render_target.texture.set_filter(FilterMode::Nearest);

            set_camera(&Camera2D {
                render_target: Some(render_target.clone()),
                target: vec2(width / 2.0, height / 2.0),
                zoom: vec2(2.0 / width, -2.0 / height),
                ..Default::default()
            });

            clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

            for y in 0..self.height {
                for x in 0..self.width {
                    let index = (y * self.width + x) as usize;

                    let gid = data.get(index).copied().unwrap_or(0);

                    if let Some((col, row, tileset)) = self.get_tile_coords(gid) {
                        let draw_x = x as f32 * tile_w + tile_w / 2.0;
                        let draw_y = y as f32 * tile_h + tile_h / 2.0;
                        tileset.spritesheet.draw_sprite(col, row, draw_x, draw_y, vec2(1.0, 1.0), false);
                    }
                }
            }

            renderer_layers.insert(layer_name.clone(), render_target);
        }

        renderer_layers
    }
}
