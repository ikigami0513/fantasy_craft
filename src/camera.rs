use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Camera {
    current_position: Vec2,
    pub lerp_factor: f32,
    pub zoom: f32
}

impl Camera {
    pub fn new(initial_position: Vec2) -> Self {
        Self {
            current_position: initial_position,
            lerp_factor: 8.0,
            zoom: 1.0
        }
    }

    pub fn update(&mut self, target_position: Vec2, dt: f32, world_size: Option<Vec2>) {
        // Smooth interpolation toward the target
        let lerp_speed = 1.0 - (-self.lerp_factor * dt).exp();
        let mut desired_position = self.current_position.lerp(target_position, lerp_speed);

        if let Some(world_size) = world_size {
            let half_view_w = screen_width() / (2.0 * self.zoom);
            let half_view_h = screen_height() / (2.0 * self.zoom);

            desired_position.x = desired_position.x.clamp(half_view_w, world_size.x - half_view_w);
            desired_position.y = desired_position.y.clamp(half_view_h, world_size.y - half_view_h);
        }

        self.current_position = desired_position;
    }
    
    pub fn set_camera(&self) {
        let camera = Camera2D {
            target: self.current_position,
            rotation: 0.0,
            zoom: vec2(self.zoom * 2.0 / screen_width(), self.zoom * 2.0 / screen_height()),
            ..Default::default()
        };
        set_camera(&camera);
    }
    
    pub fn unset_camera(&self) {
        set_default_camera();
    }
}
