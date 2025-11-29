use macroquad::prelude::*;
use parry2d::na::{Isometry2, Vector2};

/// Helper to convert Transform + Collider to Isometry2 (used by Parry)
pub fn make_isometry(position: Vec2) -> Isometry2<f32> {
    Isometry2::new(Vector2::new(position.x, position.y), 0.0)
}