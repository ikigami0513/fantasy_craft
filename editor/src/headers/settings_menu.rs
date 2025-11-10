use engine::prelude::*;

#[derive(Debug, Clone)]
pub struct SettingsMenuButton;

pub struct SettingsMenuButtonLoader;

impl ComponentLoader for SettingsMenuButtonLoader {
    fn load(&self, ctx: &mut Context, entity: hecs::Entity, _data: &serde_json::Value) {
        ctx.world.insert_one(entity, SettingsMenuButton).expect("Failed to insert SettingsMenuButton");
    }
}

pub fn settings_menu_button_clicked(ctx: &mut Context) {
    for (_, (gui_button, _)) in ctx.world.query::<(&GuiButton, &SettingsMenuButton)>().iter() {
        if gui_button.just_clicked {
            println!("Settings Menu Opening");
        }
    }
}

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.scene_loader
            .register("SettingsMenuButton", Box::new(SettingsMenuButtonLoader));

        app
            .add_system(Stage::Update, settings_menu_button_clicked);
    }
}
