use crate::{gui::systems::text_render_system, prelude::{Plugin, Stage, button_interaction_system, checkbox_logic_system, checkbox_render_system, draggable_system, gui_box_render_system, gui_image_render_system, input_field_focus_system, input_field_render_system, input_field_typing_system, input_focus_update_system, slider_interaction_system, slider_render_system}};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::Update, button_interaction_system)
            .add_system(Stage::Update, checkbox_logic_system)
            .add_system(Stage::Update, draggable_system)
            .add_system(Stage::Update, slider_interaction_system)
            .add_system(Stage::Update, input_field_focus_system)
            .add_system(Stage::Update, input_field_typing_system)
            .add_system(Stage::Update, input_focus_update_system)
            .add_system(Stage::GuiRender, gui_box_render_system)
            .add_system(Stage::GuiRender, checkbox_render_system)
            .add_system(Stage::GuiRender, slider_render_system)
            .add_system(Stage::GuiRender, text_render_system)
            .add_system(Stage::GuiRender, input_field_render_system)
            .add_system(Stage::GuiRender, gui_image_render_system);
    }
}
