use hecs::{Entity, World};
use macroquad::math::{Vec2, vec2};

use crate::scene::scene_loader::ComponentLoader;

#[derive(Debug)]
pub struct Parent(pub Entity);

pub fn find_children(world: &World, parent_id: Entity) -> Vec<Entity> {
    world.query::<&Parent>()
        .iter()
        .filter_map(|(child_entity, parent_component)| {
            if parent_component.0 == parent_id {
                Some(child_entity)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LocalOffset(pub Vec2);

pub struct LoacalOffsetLoader;

impl ComponentLoader for LoacalOffsetLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let (x, y): (f32, f32) = serde_json::from_value(data.clone())
            .unwrap_or((0.0, 0.0));

        let component = LocalOffset(vec2(x, y));

        ctx.world.insert_one(entity, component).expect("Failed to insert LocalOffset");
    }
}
