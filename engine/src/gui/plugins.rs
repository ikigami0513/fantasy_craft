use crate::{gui::systems::text_render_system, prelude::{FontComponentLoader, GameState, GuiBoxLoader, GuiButtonLoader, GuiCheckboxLoader, GuiDraggableLoader, GuiImageLoader, GuiInputFieldLoader, GuiLayoutLoader, GuiLocalOffsetLoader, GuiSliderLoader, HorizontalAlignmentLoader, Plugin, Stage, System, TextDisplayLoader, VerticalAlignmentLoader, button_interaction_system, checkbox_logic_system, checkbox_render_system, draggable_system, gui_box_render_system, gui_resolve_layout_system, gui_image_render_system, input_field_focus_system, input_field_render_system, input_field_typing_system, input_focus_update_system, slider_interaction_system, slider_render_system}};

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
            .add_system(Stage::Update, System::new(
                gui_resolve_layout_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                button_interaction_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                checkbox_logic_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                draggable_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                slider_interaction_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                input_field_focus_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                input_field_typing_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                input_focus_update_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::GuiRender, System::new(
                gui_box_render_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::GuiRender, System::new(
                checkbox_render_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::GuiRender, System::new(
                slider_render_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::GuiRender, System::new(
                text_render_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::GuiRender, System::new(
                input_field_render_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::GuiRender, System::new(
                gui_image_render_system,
                vec![GameState::Playing, GameState::Menu]
            ));
    }
}
