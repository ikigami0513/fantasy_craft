use crate::{hierarchy::systems::{hierarchy_transform_update_system, hierarchy_visible_update_system}, prelude::{Plugin, Stage}};

pub struct HierarchyPlugin;

impl Plugin for HierarchyPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::PostUpdate, hierarchy_transform_update_system)
            .add_system(Stage::PostUpdate, hierarchy_visible_update_system);
    }
}
