use crate::prelude::ComponentLoader;

#[derive(Debug)]
pub struct FontComponent(pub String);

pub struct FontComponentLoader;

impl ComponentLoader for FontComponentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = FontComponent(loader_data);

        ctx.world.insert_one(entity, component).expect("Failed to insert FontComponent");
    }
}
