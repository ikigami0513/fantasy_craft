use crate::{gui::systems::text_render_system, prelude::{button_interaction_system, gui_box_render_system, Plugin, Stage}};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::Update, button_interaction_system)
            .add_system(Stage::GuiRender, gui_box_render_system)
            .add_system(Stage::GuiRender, text_render_system);
    }
}
