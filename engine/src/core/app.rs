use hecs::World;
use macroquad::prelude::*;
use crate::core::context::Context;
use crate::core::focus::InputFocus;
use crate::core::schedule::{Schedule, Stage, System};
use crate::core::asset_server::AssetServer;
use crate::core::plugins::Plugin;
use crate::graphics::splash_screen::{despawn_splash_screen, setup_splash_screen};
use crate::prelude::Spritesheet;

pub struct App {
    pub context: Context,
    pub schedule: Schedule,
    pub window_conf: Conf
}

impl App {
    pub fn new(conf: Conf) -> Self {
        let world = World::new();
        let asset_server = AssetServer::new();

        App {
            context: Context {
                world,
                asset_server,
                dt: 0.0,
                collision_events: Vec::new(),
                prev_mouse_pos: Vec2::ZERO,
                input_focus: InputFocus::default()
            },
            schedule: Schedule::new(),
            window_conf: conf
        }
    }

    pub fn add_system(&mut self, stage: Stage, system: System) -> &mut Self {
        self.schedule.add_system(stage, system);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);
        self
    }

    pub async fn run(mut self) {
        const SPLASH_DURATION: f64 = 3.0;

        self.context.asset_server.add_spritesheet("logo_engine".to_string(), Spritesheet::new(
            load_texture("resources/textures/logo_engine.png").await.unwrap(),
            1024.0, 1024.0
        ));

        setup_splash_screen(&mut self.context);

        let start_time = get_time();
        while get_time() - start_time < SPLASH_DURATION {
            self.context.dt = get_frame_time();
            
            clear_background(BLACK);

            self.schedule.run_stage(Stage::Render, &mut self.context);
            self.schedule.run_stage(Stage::PostRender, &mut self.context);

            set_default_camera();

            self.schedule.run_stage(Stage::GuiRender, &mut self.context);

            next_frame().await;
        }

        despawn_splash_screen(&mut self.context);

        self.context.asset_server
            .load_assets_from_file("resources/assets.json")
            .await
            .expect("Failed to load assets from JSON file");

        self.schedule.run_stage(Stage::StartUp, &mut self.context);

        loop {
            self.context.dt = get_frame_time();

            clear_background(LIGHTGRAY);

            self.schedule.run_stage(Stage::Update, &mut self.context);
            self.schedule.run_stage(Stage::PostUpdate, &mut self.context);
            self.schedule.run_stage(Stage::Render, &mut self.context);
            self.schedule.run_stage(Stage::PostRender, &mut self.context);

            set_default_camera();

            self.schedule.run_stage(Stage::GuiRender, &mut self.context);

            self.context.collision_events.clear();
            self.context.prev_mouse_pos = mouse_position().into();

            next_frame().await
        }
    }
}