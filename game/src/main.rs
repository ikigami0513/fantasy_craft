use macroquad::prelude::*;
use engine::prelude::*;

mod components;
mod systems;
mod plugins;

use crate::plugins::{NpcPlugin, PlayerPlugin};
use crate::systems::{setup_system, fps_display_update, click_me_system};

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

    app
        .add_plugin(Default2dPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_system(Stage::StartUp, setup_system)
        .add_system(Stage::Update, fps_display_update)
        .add_system(Stage::Update, click_me_system);

    app.run().await
}
