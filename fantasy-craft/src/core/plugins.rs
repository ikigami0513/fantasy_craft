use crate::{audio::plugin::AudioPlugin, core::{app::App, event::EventBus, time::DeltaTime}, hierarchy::plugins::HierarchyPlugin, input::plugin::InputPlugin, prelude::{AnimationPlugin, Camera2dPlugin, GameState, GuiPlugin, PhysicsPlugin, Stage, System, TiledMapPlugin, collider_debug_render_system}, utils::plugins::UtilsPlugin};

pub trait Plugin {
    fn build(&self, app: &mut App);
}

pub struct Default2dPlugin;

impl Plugin for Default2dPlugin {
    fn build(&self, app: &mut App) {
        app.context.insert_resource(EventBus::new());
        app.context.insert_resource(DeltaTime(0.0));

        app
            .add_plugin(UtilsPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(Camera2dPlugin)
            .add_plugin(HierarchyPlugin)
            .add_plugin(TiledMapPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(GuiPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(InputPlugin);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Stage::PostRender, System::new(
                collider_debug_render_system,
                vec![GameState::Playing]
            ));
    }
}
