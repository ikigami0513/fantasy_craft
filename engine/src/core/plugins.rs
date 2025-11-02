use crate::{core::app::App, hierarchy::plugins::HierarchyPlugin, prelude::{AnimationPlugin, Camera2dPlugin, GuiPlugin, PhysicsPlugin, Stage, TiledMapPlugin, collider_debug_render_system}};

pub trait Plugin {
    fn build(&self, app: &mut App);
}

pub struct Default2dPlugin;

impl Plugin for Default2dPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PhysicsPlugin)
            .add_plugin(Camera2dPlugin)
            .add_plugin(HierarchyPlugin)
            .add_plugin(TiledMapPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(GuiPlugin);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Stage::PostRender, collider_debug_render_system);
    }
}
