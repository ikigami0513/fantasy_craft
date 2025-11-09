use crate::{gui::systems::text_render_system, prelude::{FontComponentLoader, GuiBoxLoader, GuiButtonLoader, GuiCheckboxLoader, GuiDraggableLoader, GuiImageLoader, GuiInputFieldLoader, GuiLayoutLoader, GuiLocalOffsetLoader, GuiSliderLoader, HorizontalAlignmentLoader, Plugin, Stage, TextDisplayLoader, VerticalAlignmentLoader, button_interaction_system, checkbox_logic_system, checkbox_render_system, draggable_system, gui_box_render_system, gui_hierarchy_transform_update_system, gui_image_render_system, gui_layout_system, input_field_focus_system, input_field_render_system, input_field_typing_system, input_focus_update_system, slider_interaction_system, slider_render_system}};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.scene_loader
            .register("GuiLayout", Box::new(GuiLayoutLoader))
            .register("GuiLocalOffset", Box::new(GuiLocalOffsetLoader))
            .register("FontComponent", Box::new(FontComponentLoader))
            .register("VerticalAlignment", Box::new(VerticalAlignmentLoader))
            .register("HorizontalAlignment", Box::new(HorizontalAlignmentLoader))
            .register("TextDisplay", Box::new(TextDisplayLoader))
            .register("GuiBox", Box::new(GuiBoxLoader))
            .register("GuiButton", Box::new(GuiButtonLoader))
            .register("GuiDraggable", Box::new(GuiDraggableLoader))
            .register("GuiSlider", Box::new(GuiSliderLoader))
            .register("GuiCheckbox", Box::new(GuiCheckboxLoader))
            .register("GuiInputField", Box::new(GuiInputFieldLoader))
            .register("GuiImage", Box::new(GuiImageLoader));

        app
            .add_system(Stage::Update, gui_layout_system)
            .add_system(Stage::Update, gui_hierarchy_transform_update_system)
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
