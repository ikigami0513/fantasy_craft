use engine::scene::scene_loader::ComponentLoader;
use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Behavior {
    Stand,
    Wander
}

#[allow(dead_code)]
impl Behavior {
    pub fn to_str(&self) -> &'static str {
        match self {
            Behavior::Stand => "stand",
            Behavior::Wander => "wander"
        }
    }

    pub fn from_str(value: &str) -> Behavior {
        match value {
            "stand" => Behavior::Stand,
            "wander" => Behavior::Wander,
            _ => Behavior::Stand
        }
    }
}

#[derive(Debug)]
pub struct BehaviorComponent(pub Behavior);

pub struct BehaviorComponentLoader;

impl ComponentLoader for BehaviorComponentLoader {
    fn load(&self, ctx: &mut engine::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = BehaviorComponent(Behavior::from_str(&loader_data));

        ctx.world.insert_one(entity, component).expect("Failed to insert BehaviorComponent")
    }
}

#[derive(Debug)]
pub struct PlayerTag;

pub struct PlayerTagLoader;

impl ComponentLoader for PlayerTagLoader {
    fn load(&self, ctx: &mut engine::prelude::Context, entity: hecs::Entity, _data: &serde_json::Value) {
        ctx.world.insert_one(entity, PlayerTag).expect("Failed to insert PlayerTag");
    }
}

#[derive(Debug)]
pub struct AnimationPrefix(pub String);

pub struct AnimationPrefixLoader;

impl ComponentLoader for AnimationPrefixLoader {
    fn load(&self, ctx: &mut engine::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = AnimationPrefix(loader_data);

        ctx.world.insert_one(entity, component).expect("Failed to insert AnimationPrefix");
    }
}

#[derive(Debug)]
pub struct NpcTag {
    pub name: String,
    pub wander_time: f32,
    pub wander_target_duration: f32
}

#[derive(Deserialize, Debug, Default)]
pub struct NpcTagLoaderData {
    pub name: String,
    pub wander_time: f32,
    pub wander_target_duration: f32
}

pub struct NpcTagLoader;

impl ComponentLoader for NpcTagLoader {
    fn load(&self, ctx: &mut engine::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: NpcTagLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = NpcTag {
            name: loader_data.name,
            wander_time: loader_data.wander_time,
            wander_target_duration: loader_data.wander_target_duration
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert NpcTag");
    }
}

#[derive(Debug)]
pub struct FpsDisplay {
    pub fps_timer: f32,
    pub displayed_fps: i32
}

#[derive(Debug, Default)]
pub struct ClickMeAction;
