use macroquad::prelude::*;

// We need to register the custom getrandom handler to satisfy the dependency
// without pulling in wasm-bindgen JS glue code.
fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    for b in buf.iter_mut() {
        // Macroquad's rand::rand() returns a u32, we cast it to u8.
        // Note: This is a pseudo-RNG, usually sufficient for games but not for cryptography.
        *b = macroquad::rand::rand() as u8;
    }
    Ok(())
}

// This macro registers our function as the source of randomness for the whole compilation unit.
getrandom::register_custom_getrandom!(custom_getrandom);

// UPDATED: We use the crate name 'fantasy_craft' (with an underscore)
// because Rust replaces hyphens with underscores in package names.
use fantasy_craft::prelude::*;

mod components;
mod systems;
mod plugins;

use crate::components::FpsDisplayLoader;
use crate::plugins::{NpcPlugin, PlayerPlugin};
use crate::systems::fps_display_update;

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
