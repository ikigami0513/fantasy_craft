use std::sync::Arc;
use macroquad::math::Vec2;

use crate::core::plugins::Plugin;
use crate::prelude::Stage;
use crate::{graphics::sprites::Spritesheet, prelude::Context};
use crate::physics::components::Transform;

pub struct AnimationKeyFrame {
    pub column: u32,
    pub row: u32
}

impl AnimationKeyFrame {
    pub fn new(column: u32, row: u32) -> Self {
        Self { column, row }
    }
}

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

    pub fn draw(&self, x: f32, y: f32, scale: Vec2) {
        let frame_index = self.current_index.floor() as usize;
        let key_frame = self.frames.get(frame_index).expect("AnimationKeyFrame index out of bounds");
        self.spritesheet.draw_sprite(key_frame.column, key_frame.row, x, y, scale, self.flip);
    }
}

#[derive(Debug)]
pub struct AnimationComponent(pub String);

pub fn update_animations(ctx: &mut Context) {
    for (_, animation_comp) in ctx.world.query::<&AnimationComponent>().iter() {
        if let Some(animation) = ctx.asset_server.get_animation_mut(&animation_comp.0) {
            animation.update(ctx.dt);
        }
    }
}

pub fn animation_render_system(ctx: &mut Context) {
    for (_, (animation_comp, transform)) in ctx.world.query::<(&AnimationComponent, &Transform)>().iter() {
        if let Some(animation) = ctx.asset_server.get_animation_mut(&animation_comp.0) {
            animation.draw(transform.position.x, transform.position.y, transform.scale);
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::Update, update_animations)
            .add_system(Stage::Render, animation_render_system);
    }
}
