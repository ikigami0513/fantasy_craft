use macroquad::prelude::*;
use parry2d::shape::{SharedShape, Cuboid};
use parry2d::na::Vector2;
use serde::Deserialize;

use crate::scene::scene_loader::ComponentLoader;

#[derive(Deserialize, Debug, Default)]
pub struct Vec2Data {
    pub x: f32,
    pub y: f32
}

#[derive(Deserialize, Debug, Default)]
pub struct UVec2Data {
    pub x: u32,
    pub y: u32
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: Vec2,
    pub scale: Vec2
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            rotation: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0)
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct TransformLoaderData {
    #[serde(default)]
    pub position: Vec2Data,

    #[serde(default)]
    pub rotation: Vec2Data,

    #[serde(default)]
    pub scale: Vec2Data
}

pub struct TransformLoader;

impl ComponentLoader for TransformLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: TransformLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = Transform {
            position: vec2(loader_data.position.x, loader_data.position.y),
            rotation: vec2(loader_data.rotation.x, loader_data.rotation.y),
            scale: vec2(loader_data.scale.x, loader_data.scale.y)
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert Transform");
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub body_type: BodyType
}

impl RigidBody {
    pub fn new(body_type: BodyType) -> Self {
        Self {
            body_type
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct RigidBodyLoaderData {
    pub body_type: String
}

pub struct RigidBodyLoader;

impl ComponentLoader for RigidBodyLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: RigidBodyLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let body_type = match loader_data.body_type.as_str() {
            "static" => BodyType::Static,
            "dynamic" => BodyType::Dynamic,
            "kinematic" => BodyType::Kinematic,
            _ => {
                println!("Warning: Type de corps non reconnu '{}', utilisation de Dynamic par défaut.", loader_data.body_type);
                BodyType::Dynamic
            }
        };

        let component = RigidBody {
            body_type
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert RigidBody");
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Collider {
    pub shape: SharedShape,
    pub half_extents: Vec2
}

impl Collider {
    pub fn new_box(width: f32, height: f32) -> Self {
        Self {
            shape: SharedShape::new(Cuboid::new(Vector2::new(width / 2.0, height / 2.0))),
            half_extents: vec2(width / 2.0, height / 2.0)
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ColliderLoaderData {
    pub shape: String,
    pub width: f32,
    pub height: f32
}

pub struct ColliderLoader;

impl ComponentLoader for ColliderLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: ColliderLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        match loader_data.shape.as_str() {
            "Box" => {
                ctx.world.insert_one(entity, Collider::new_box(loader_data.width, loader_data.height)).expect("Failed to insert Collider shape Box");
            },
            _ => {
                println!("Warning: Type de shape non reconnu '{}', utilisation de Box par défaut.", loader_data.shape);
                ctx.world.insert_one(entity, Collider::new_box(loader_data.width, loader_data.height)).expect("Failed to insert Collider shape Box");
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Velocity(pub Vec2);

pub struct VelocityLoader;

impl ComponentLoader for VelocityLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: Vec2Data = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = Velocity(vec2(loader_data.x, loader_data.y));

        ctx.world.insert_one(entity, component).expect("Failed to insert Velocity");
    }
}

#[derive(Debug, Clone)]
pub struct Speed(pub f32);

pub struct SpeedLoader;

impl ComponentLoader for SpeedLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: f32 = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = Speed(loader_data);

        ctx.world.insert_one(entity, component).expect("Failed to insert Speed");
    }
}
