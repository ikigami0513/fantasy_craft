use macroquad::prelude::*;
use engine::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Editor".to_owned(),
        window_width: 3840,
        window_height: 2160,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new(window_conf());

    app
        .with_splash_screen_enabled(false)
        .with_assets_file("resources/editor/assets.json".to_string())
        .with_scene_path("resources/editor/scene.json".to_string())
        .add_plugin(Default2dPlugin);

    app.run().await;
}
