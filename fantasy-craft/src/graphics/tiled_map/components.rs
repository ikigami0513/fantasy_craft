use hecs::Entity;
use serde::Deserialize;
use serde_json::Value;

use crate::scene::scene_loader::ComponentLoader;

#[derive(Debug)]
pub struct TileMapComponent(pub String);

pub struct TileMapComponentLoader;

impl ComponentLoader for TileMapComponentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &Value) {
        let map_name: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = TileMapComponent(map_name);

        ctx.world.insert_one(entity, component).expect("Failed to insert TileMapComponent");
    }
}

#[derive(Debug)]
pub struct TileMapLayerComponent {
    pub tilemap_name: String,
    pub layer_name: String
}

#[derive(Deserialize, Debug, Default)]
struct TileMapLayerComponentLoaderData {
    #[serde(default)]
    pub tilemap_name: String,

    #[serde(default)]
    pub layer_name: String
}

pub struct TileMapLayerComponentLoader;

impl ComponentLoader for TileMapLayerComponentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &Value) {
        let loader_data: TileMapLayerComponentLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = TileMapLayerComponent {
            tilemap_name: loader_data.tilemap_name,
            layer_name: loader_data.layer_name
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert TileMapLayerComponent");
    }
}

#[derive(Debug)]
pub struct MainTileMap;

pub struct MainTileMapLoader;

impl ComponentLoader for MainTileMapLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, _data: &Value) {
        ctx.world.insert_one(entity, MainTileMap).expect("Failed to insert MainTileMap");
    }
}