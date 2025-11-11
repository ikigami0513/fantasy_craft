use crate::{hierarchy::systems::{hierarchy_transform_update_system, hierarchy_visible_update_system}, prelude::{GameState, LocalOffsetLoader, Plugin, Stage, System}};

pub struct HierarchyPlugin;

impl Plugin for HierarchyPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.scene_loader
            .register("LocalOffset", Box::new(LocalOffsetLoader));

        app
            .add_system(Stage::PostUpdate, System::new(
                hierarchy_transform_update_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::PostUpdate, System::new(
                hierarchy_visible_update_system,
                vec![GameState::Playing, GameState::Menu]
            ));
    }
}
