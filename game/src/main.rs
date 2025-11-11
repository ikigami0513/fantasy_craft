use macroquad::prelude::*;
use engine::prelude::*;

mod components;
mod systems;
mod plugins;

use crate::components::{ClickMeActionLoader, FpsDisplayLoader};
use crate::plugins::{NpcPlugin, PlayerPlugin};
use crate::systems::{click_me_system, fps_display_update};

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
        .register("FpsDisplay", Box::new(FpsDisplayLoader))
        .register("ClickMeAction", Box::new(ClickMeActionLoader));

    app
        .with_splash_screen_enabled(false)
        .with_assets_file("resources/assets.json".to_string())
        .with_scene_path("resources/scenes/dev.json".to_string())
        .add_plugin(Default2dPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_system(Stage::Update, System::new(
            fps_display_update,
            vec![GameState::Playing]
        ))
        .add_system(Stage::Update, System::new(
            click_me_system, vec![GameState::Playing]
        ));

    app.run().await
}
