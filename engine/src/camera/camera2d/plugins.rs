use crate::{core::plugins::Plugin, prelude::{CameraComponentLoader, CameraTargetLoader, GameState, MainCameraLoader, Stage, System, update_camera}};

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.scene_loader
            .register("CameraComponent", Box::new(CameraComponentLoader))
            .register("MainCamera", Box::new(MainCameraLoader))
            .register("CameraTarget", Box::new(CameraTargetLoader));

        app
            .add_system(Stage::PostUpdate, System::new(
                update_camera,
                vec![GameState::Playing]
            ));
    }
}
