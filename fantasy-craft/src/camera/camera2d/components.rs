use hecs::Entity;
use macroquad::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use crate::core::context::Context;
use crate::scene::scene_loader::ComponentLoader;

#[derive(Debug)]
pub struct CameraComponent {
    pub lerp_factor: f32,
    pub zoom: f32,
    pub camera: Camera2D
}

fn default_lerp() -> f32 { 0.0 }
fn default_zoom() -> f32 { 1.0 }

#[derive(Deserialize, Debug, Default)]
struct CameraComponentLoaderData {
    #[serde(default="default_lerp")]
    pub lerp_factor: f32,

    #[serde(default="default_zoom")]
    pub zoom: f32
}

pub struct CameraComponentLoader;

impl ComponentLoader for CameraComponentLoader {
    fn load(&self, ctx: &mut Context, entity: Entity, data: &Value) {
        let loader_data: CameraComponentLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = CameraComponent {
            lerp_factor: loader_data.lerp_factor,
            zoom: loader_data.zoom,
            camera: Camera2D::default()
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert CameraComponent");
    }
}

#[derive(Debug)]
pub struct MainCamera;

pub struct MainCameraLoader;

impl ComponentLoader for MainCameraLoader {
    fn load(&self, ctx: &mut Context, entity: Entity, _data: &Value) {
        ctx.world.insert_one(entity, MainCamera).expect("Failed to insert MainCamera");
    }
}

#[derive(Debug)]
pub struct CameraTarget;

pub struct CameraTargetLoader;

impl ComponentLoader for CameraTargetLoader {
    fn load(&self, ctx: &mut Context, entity: Entity, _data: &Value) {
        ctx.world.insert_one(entity, CameraTarget).expect("Failed to insert CameraTarget");
    }
}
