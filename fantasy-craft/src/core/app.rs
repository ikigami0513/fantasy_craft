use hecs::World;
use macroquad::prelude::*;
use futures::{FutureExt, future::BoxFuture};
use crate::core::context::Context;
use crate::core::event::EventBus;
use crate::core::schedule::{Schedule, Stage};
use crate::core::asset_server::AssetServer;
use crate::core::plugins::Plugin;
use crate::core::time::DeltaTime;
use crate::core::web_context::WebContext;
use crate::graphics::splash_screen::{SplashScreenData, animate_splash_screen, despawn_splash_screen, setup_splash_screen};
use crate::input::manager::InputManager;
use crate::prelude::{Spritesheet, System};
use crate::gui::resources::PreviousMousePosition;
use crate::scene::scene_loader::SceneLoader;

pub struct App {
    pub context: Context,
    pub schedule: Schedule,
    pub scene_loader: SceneLoader,
    pub window_conf: Conf,
    pub scene_path: Option<String>,
    pub assets_file: Option<String>,
    show_splash_screen: bool,
    splash_screen_logo: String,
    splash_screen_background_color: Color,
    binding_path: Option<String>
}

impl App {
    pub fn new(conf: Conf) -> Self {
        let world = World::new();
        let asset_server = AssetServer::new();

        App {
            context: Context::new(
                world,
                asset_server
            ),
            schedule: Schedule::new(),
            scene_loader: SceneLoader::new(),
            window_conf: conf,
            scene_path: None,
            show_splash_screen: true,
            assets_file: None,
            splash_screen_logo: "resources/textures/logo_engine.png".to_string(),
            splash_screen_background_color: Color::new(1.0, 0.980392157, 0.960784314, 1.0),
            binding_path: None
        }
    }

    pub fn with_splash_screen_enabled(&mut self, enabled: bool) -> &mut Self {
        self.show_splash_screen = enabled;
        self
    }

    pub fn with_splash_screen_logo(&mut self, path: &str) -> &mut Self {
        self.splash_screen_logo = path.to_string();
        self
    }

    pub fn with_splash_screen_background_color(&mut self, color: Color) -> &mut Self {
        self.splash_screen_background_color = color;
        self
    }

    pub fn with_scene_path(&mut self, scene_path: String) -> &mut Self {
        self.scene_path = Some(scene_path);
        self
    }

    pub fn with_assets_file(&mut self, file_path: String) -> &mut Self {
        self.assets_file = Some(file_path);
        self
    }

    pub fn with_binding_file(&mut self, file_path: String) -> &mut Self {
        self.binding_path = Some(file_path);
        self
    }

    pub fn add_system(&mut self, stage: Stage, system: System) -> &mut Self {
        self.schedule.add_system(stage, system);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);
        self
    }

    fn resolve_path(base: &str, path: &str) -> String {
        if path.starts_with("http") {
            return path.to_string();
        }
        format!("{}{}", base, path)
    }

    pub async fn run(mut self) {
        // 1. Fetch the Base URL from JavaScript
        let base_url = WebContext::get_base_url();
        info!("App: Resolved Base URL: {}", base_url);

        // 2. Resolve paths using the base URL
        let resolved_splash_path = Self::resolve_path(&base_url, &self.splash_screen_logo);
        
        // Resolve optional paths
        let resolved_assets_file = self.assets_file.as_ref().map(|p| Self::resolve_path(&base_url, p));
        let resolved_scene_path = self.scene_path.as_ref().map(|p| Self::resolve_path(&base_url, p));
        let resolved_binding_path = self.binding_path.as_ref().map(|p| Self::resolve_path(&base_url, p));

        const SPLASH_MIN_DURATION: f64 = 3.0;

        let mut maybe_asset_server: Option<AssetServer> = None;

        if self.show_splash_screen {
            let splash_texture = match load_texture(&resolved_splash_path).await {
                Ok(tex) => tex,
                Err(e) => {
                    error!("Failed to load splash logo at: {}. Error: {}", resolved_splash_path, e);
                    // Generate a 1x1 Magenta texture as fallback
                    let img = Image::gen_image_color(1, 1, MAGENTA);
                    Texture2D::from_image(&img)
                }
            };

            // --- Splash setup ---
            self.context.asset_server.add_spritesheet(
                "splash_screen_logo".to_string(),
                Spritesheet::new(
                    splash_texture,
                    1024.0,
                    1024.0,
                ),
            );

            self.context.insert_resource(SplashScreenData {
                background_color: self.splash_screen_background_color
            });

            setup_splash_screen(&mut self.context);
            let start_time = get_time();

            // --- Création d'une future pour le chargement ---
            let mut loading_asset_server = AssetServer::new();

            let asset_path_for_future = resolved_assets_file.clone();

            let mut load_future: BoxFuture<'static, (AssetServer, Result<(), Box<dyn std::error::Error>>)> =
                Box::pin(async move {
                    let result = if let Some(path) = asset_path_for_future {
                        loading_asset_server.load_assets_from_file(&path).await
                    } else {
                        Ok(())
                    };
                    (loading_asset_server, result)
                });

            let mut assets_loaded = false;

            // --- Boucle du splash ---
            loop {
                let dt = self.context.resource_mut::<DeltaTime>();
                dt.0 = get_frame_time();
                clear_background(self.splash_screen_background_color);

                // Animation + rendu
                animate_splash_screen(&mut self.context);
                self.schedule.run_stage(Stage::Render, &mut self.context);
                self.schedule.run_stage(Stage::PostRender, &mut self.context);
                set_default_camera();
                self.schedule.run_stage(Stage::GuiRender, &mut self.context);

                next_frame().await;

                let elapsed = get_time() - start_time;
                let duration_done = elapsed >= SPLASH_MIN_DURATION;

                if !assets_loaded {
                    if let Some((loaded_server, result)) = load_future.as_mut().now_or_never() {
                        // Log error instead of crashing if assets.json is missing
                        if let Err(e) = result {
                             error!("Failed to load assets from JSON file: {}", e);
                        }
                        maybe_asset_server = Some(loaded_server);
                        assets_loaded = true;
                    }
                }

                if assets_loaded && duration_done {
                    break;
                }
            }

            despawn_splash_screen(&mut self.context);
        } else {
            // --- No splash: Load assets directly ---
            let mut asset_server = AssetServer::new();

            if let Some(path) = &resolved_assets_file {
                if let Err(e) = asset_server.load_assets_from_file(path).await {
                     error!("Failed to load assets from JSON file: {}", e);
                }
            }

            maybe_asset_server = Some(asset_server);
        }

        // Réinjection du AssetServer chargé
        if let Some(loaded_server) = maybe_asset_server {
            self.context.asset_server.merge(loaded_server);
            self.context.asset_server.finalize_textures().await;
        }

        self.context.asset_server.prepare_loaded_tiledmap().await;

        if let Some(scene_path) = resolved_scene_path {
            self.scene_loader.load_scene_from_file(&scene_path, &mut self.context).await.unwrap();
        }

        if let Some(binding_path) = resolved_binding_path {
            info!("App: Loading bindings from: {}", binding_path);
            
            // On utilise load_string pour le support WASM/HTTP
            match load_string(&binding_path).await {
                Ok(content) => {
                    if let Some(input_manager) = self.context.get_resource_mut::<InputManager>() {
                        // On passe le contenu, plus le chemin !
                        input_manager.load_from_string(&content);
                    }
                },
                Err(e) => {
                    error!("App: Failed to load binding file '{}': {}", binding_path, e);
                }
            }
        }

        // --- Démarrage du jeu ---
        self.schedule.run_stage(Stage::StartUp, &mut self.context);

        loop {
            let dt = self.context.resource_mut::<DeltaTime>();
            dt.0 = get_frame_time();
            clear_background(LIGHTGRAY);

            self.schedule.run_stage(Stage::Update, &mut self.context);
            self.schedule.run_stage(Stage::PostUpdate, &mut self.context);
            self.schedule.run_stage(Stage::Render, &mut self.context);
            self.schedule.run_stage(Stage::PostRender, &mut self.context);

            set_default_camera();
            self.schedule.run_stage(Stage::GuiRender, &mut self.context);

            if let Some(event_bus) = self.context.get_resource_mut::<EventBus>() {
                event_bus.clear();
            }

            if let Some(prev_mouse_pos) = self.context.get_resource_mut::<PreviousMousePosition>() {
                prev_mouse_pos.0 = mouse_position().into();
            }

            next_frame().await;
        }
    }
}
