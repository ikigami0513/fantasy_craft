use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct ColorData {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}
