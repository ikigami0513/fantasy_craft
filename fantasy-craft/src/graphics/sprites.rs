use macroquad::prelude::*;

pub struct Spritesheet {
    pub texture: Texture2D,
    pub sprite_width: f32,
    pub sprite_height: f32
}

impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_width: f32, sprite_height: f32) -> Self {
        Self {
            texture,
            sprite_width,
            sprite_height
        }
    }

    pub fn get_source_rect(&self, col: u32, row: u32) -> Option<Rect> {
        let x = col as f32 * self.sprite_width;
        let y = row as f32 * self.sprite_height;
        Some(Rect::new(x, y, self.sprite_width, self.sprite_height))
    }

    pub fn draw_sprite(&self, col: u32, row: u32, x: f32, y: f32, scale: Vec2, flip_x: bool) {
        let source_rect = self.get_source_rect(col, row);

        let dest_width = self.sprite_width * scale.x;
        let dest_height = self.sprite_height * scale.y;

        let draw_x = x - dest_width / 2.0;
        let draw_y = y - dest_height / 2.0;

        let draw_params = DrawTextureParams {
            dest_size: Some(vec2(dest_width, dest_height)),
            source: source_rect,
            flip_x,
            ..Default::default()
        };

        draw_texture_ex(&self.texture, draw_x, draw_y, WHITE, draw_params);
    }
}
