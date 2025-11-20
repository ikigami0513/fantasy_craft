use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::{gui::{alignment::{HorizontalAlignmentLoader, VerticalAlignmentLoader}, gui_action::GuiActionLoader, gui_box::{GuiBoxLoader, gui_box_render_system}, gui_button::button_interaction_system, gui_checkbox::{GuiCheckboxLoader, checkbox_logic_system, checkbox_render_system}, gui_draggable::{GuiDraggableLoader, draggable_system}, gui_image::{GuiImageLoader, gui_image_render_system}, gui_input_field::{GuiInputFieldLoader, input_field_focus_system, input_field_render_system, input_field_typing_system}, gui_layout::{GuiLayoutLoader, gui_resolve_layout_system}, gui_local_offset::GuiLocalOffsetLoader, gui_slider::{GuiSliderLoader, slider_interaction_system, slider_render_system}, text_display::{TextDisplayLoader, text_render_system}}, prelude::{GameState, Plugin, Stage, System}};
use crate::gui::resources::{UiResolvedRects, PreviousMousePosition};
use crate::gui::gui_element::GuiElementLoader;
use crate::gui::font_component::FontComponentLoader;
use crate::gui::gui_button::GuiButtonLoader;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.context.insert_resource(UiResolvedRects(HashMap::new()));
        app.context.insert_resource(PreviousMousePosition(Vec2::ZERO));

        app.scene_loader
            .register("GuiElement", Box::new(GuiElementLoader))
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
            .register("GuiImage", Box::new(GuiImageLoader))
            .register("GuiAction", Box::new(GuiActionLoader));

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
