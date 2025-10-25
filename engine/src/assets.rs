use std::sync::Arc;
use macroquad::prelude::*;

// --- AnimationKeyFrame ---
pub struct AnimationKeyFrame {
    pub column: u32,
    pub row: u32
}

impl AnimationKeyFrame {
    pub fn new(column: u32, row: u32) -> Self {
        Self { column, row }
    }
}

// --- Spritesheet ---
pub struct Spritesheet {
    texture: Texture2D,
    sprite_width: f32,
    sprite_height: f32
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

    pub fn draw_sprite(&self, col: u32, row: u32, x: f32, y: f32, scale: f32, flip_x: bool) {
        let source_rect = self.get_source_rect(col, row);

        let dest_width = self.sprite_width * scale;
        let dest_height = self.sprite_height * scale;

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

// --- Animation ---
pub struct Animation {
    pub spritesheet: Arc<Spritesheet>,
    pub frames: Vec<AnimationKeyFrame>,
    pub current_index: f32,
    pub speed: f32,
    pub flip: bool
}

impl Animation {
    pub fn new(spritesheet: Arc<Spritesheet>, frames: Vec<AnimationKeyFrame>, speed: f32, flip: bool) -> Self {
        Self {
            spritesheet,
            frames,
            current_index: 0.0,
            speed,
            flip
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.current_index += self.speed * dt;
        let num_frames = self.frames.len() as f32;

        if self.current_index >= num_frames {
            self.current_index = self.current_index.rem_euclid(num_frames);
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        let frame_index = self.current_index.floor() as usize;
        let key_frame = self.frames.get(frame_index).expect("AnimationKeyFrame index out of bounds");
        self.spritesheet.draw_sprite(key_frame.column, key_frame.row, x, y, 1.0, self.flip);
    }
}