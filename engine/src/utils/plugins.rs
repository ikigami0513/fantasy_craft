use crate::prelude::{DirectionComponentLoader, LocalVisibleLoader, Plugin, StateComponentLoader, VisibleLoader};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.scene_loader
            .register("DirectionComponent", Box::new(DirectionComponentLoader))
            .register("StateComponent", Box::new(StateComponentLoader))
            .register("Visible", Box::new(VisibleLoader))
            .register("LocalVisible", Box::new(LocalVisibleLoader));
    }
}
