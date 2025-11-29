use hecs::Entity;

use crate::prelude::ComponentLoader;

#[derive(Debug, Default)]
pub struct GuiElement;

pub struct GuiElementLoader;

impl ComponentLoader for GuiElementLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, _data: &serde_json::Value) {
        ctx.world.insert_one(entity, GuiElement).expect("Failed to insert GuiElement");
    }
}
