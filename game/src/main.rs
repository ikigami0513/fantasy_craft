use macroquad::prelude::*;
use engine::prelude::*;

mod components;
mod systems;
mod plugins;

use crate::components::{FpsDisplayLoader};
use crate::plugins::{NpcPlugin, PlayerPlugin};
use crate::systems::{fps_display_update};

fn window_conf() -> Conf {
    Conf {
        window_title: "Fantasy Craft".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new(window_conf());

    app.scene_loader
        .register("FpsDisplay", Box::new(FpsDisplayLoader));

    app
        .with_splash_screen_enabled(true)
        .with_assets_file("resources/assets.json".to_string())
        .with_scene_path("resources/scenes/dev.json".to_string())
        .with_binding_file("resources/bindings.json".to_string())
        .add_plugin(Default2dPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_system(Stage::Update, System::new(
            fps_display_update,
            vec![GameState::Playing, GameState::Menu]
        ));

    app.run().await
}
