use crate::prelude::{GameState, MainTileMapLoader, Plugin, Stage, System, TileMapComponentLoader, TileMapLayerComponentLoader, tilemap_layer_render_system};
use crate::graphics::tiled_map::systems::tilemap_render_system;

pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.scene_loader
            .register("TileMapComponent", Box::new(TileMapComponentLoader))
            .register("TileMapLayerComponent", Box::new(TileMapLayerComponentLoader))
            .register("MainTileMap", Box::new(MainTileMapLoader));

        app
            .add_system(Stage::Render, System::new(
                tilemap_layer_render_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Render, System::new(
                tilemap_render_system,
                vec![GameState::Playing, GameState::Menu]
            ));
    }
}
