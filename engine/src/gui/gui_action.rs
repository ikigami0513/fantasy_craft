use hecs::Entity;
use serde::Deserialize;

use crate::prelude::ComponentLoader;

#[derive(Debug, Clone)]
pub struct GuiAction {
    pub action_id: String
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiActionLoaderData {
    pub action_id: String
}

pub struct GuiActionLoader;

impl ComponentLoader for GuiActionLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let loader_data: GuiActionLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiAction {
            action_id: loader_data.action_id
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiAction");
    }
}
