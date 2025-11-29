use macroquad::prelude::*;
use serde::Deserialize;

use crate::{core::event::EventBus, gui::{alignment::{HorizontalAlignment, HorizontalAlignmentType, VerticalAlignment, VerticalAlignmentType}, event::UiClickEvent, gui_action::GuiAction, gui_box::GuiBox, resources::UiResolvedRects}, prelude::{ColorData, ComponentLoader, Context, Visible}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Idle,
    Hovered,
    Pressed
}

impl ButtonState {
    pub fn to_str(&self) -> &'static str {
        match self {
            ButtonState::Idle => "idle",
            ButtonState::Hovered => "hovered",
            ButtonState::Pressed => "pressed"
        }
    }

    pub fn from_str(value: &str) -> ButtonState {
        match value {
            "idle" => ButtonState::Idle,
            "hovered" => ButtonState::Hovered,
            "pressed" => ButtonState::Pressed,
            _ => ButtonState::Idle
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GuiButton {
    pub state: ButtonState,
    pub just_clicked: bool,
    pub hovered_color: Color,
    pub pressed_color: Color,
    pub normal_color: Color
}

impl Default for GuiButton {
    fn default() -> Self {
        Self {
            state: ButtonState::Idle,
            just_clicked: false,
            hovered_color: Color::new(0.0, 0.0, 0.0, 1.0),
            pressed_color: Color::new(0.0, 0.0, 0.0, 1.0),
            normal_color: Color::new(0.0, 0.0, 0.0, 1.0)
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiButtonLoaderData {
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub just_clicked: bool,
    
    pub hovered_color: ColorData,
    pub pressed_color: ColorData,
    pub normal_color: ColorData
}

pub struct GuiButtonLoader;

impl ComponentLoader for GuiButtonLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiButtonLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiButton {
            state: ButtonState::from_str(loader_data.state.as_str()),
            just_clicked: loader_data.just_clicked,
            hovered_color: Color::new(
                loader_data.hovered_color.r,
                loader_data.hovered_color.g,
                loader_data.hovered_color.b,
                loader_data.hovered_color.a
            ),
            pressed_color: Color::new(
                loader_data.pressed_color.r,
                loader_data.pressed_color.g,
                loader_data.pressed_color.b,
                loader_data.pressed_color.a
            ),
            normal_color: Color::new(
                loader_data.normal_color.r,
                loader_data.normal_color.g,
                loader_data.normal_color.b,
                loader_data.normal_color.a
            )
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiButton");
    }
}

pub fn button_interaction_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_down(MouseButton::Left);
    let just_clicked = is_mouse_button_pressed(MouseButton::Left);

    let (world, resources) = (&mut ctx.world, &mut ctx.resources);

    // --- READ PHASE ---
    // We get the read-only resource first.
    let resolved_rects_map = &resources.get::<UiResolvedRects>()
        .expect("UiResolvedRects resource is missing")
        .0;

    // We create a local buffer to store events because we can't 
    // borrow EventBus mutably while holding resolved_rects_map.
    let mut events_to_send: Vec<UiClickEvent> = Vec::new();

    let mut query = world.query::<(
        &mut GuiButton, 
        &GuiBox, 
        Option<&GuiAction>, 
        Option<&Visible>, 
        Option<&HorizontalAlignment>, 
        Option<&VerticalAlignment>
    )>();

    for (entity, (button, gui_box, action_opt, visibility, h_align, v_align)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        button.just_clicked = false;

        // We use the read-only map here
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue; 
            };

        if !gui_box.screen_space { continue; }

        let mut x = resolved_pos.x;
        let mut y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        // Apply alignment
        if let Some(h_align) = h_align {
            match h_align.0 {
                HorizontalAlignmentType::Left => {},
                HorizontalAlignmentType::Center => x -= w / 2.0,
                HorizontalAlignmentType::Right => x -= w,
            }
        }
        
        if let Some(v_align) = v_align {
            match v_align.0 {
                VerticalAlignmentType::Top => {},
                VerticalAlignmentType::Center => y -= h / 2.0,
                VerticalAlignmentType::Bottom => y -= h,
            }
        }

        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        match button.state {
            ButtonState::Idle => {
                if is_hovered { button.state = ButtonState::Hovered; }
            }
            ButtonState::Hovered => {
                if !is_hovered { button.state = ButtonState::Idle; }
                else if just_clicked { button.state = ButtonState::Pressed; }
            }
            ButtonState::Pressed => {
                if !is_pressed {
                    if is_hovered {
                        button.just_clicked = true;
                        button.state = ButtonState::Hovered;

                        // --- COLLECT PHASE ---
                        // Instead of sending immediately, we push to the buffer.
                        if let Some(action) = action_opt {
                            events_to_send.push(UiClickEvent {
                                action_id: action.action_id.clone(),
                                entity,
                            });
                        }
                    } else {
                        button.state = ButtonState::Idle;
                    }
                }
            }
        }
    }

    // --- SEND PHASE ---
    // The loop is done, so `resolved_rects_map` borrow is dropped (or can be inferred dropped).
    // We are now free to borrow `resources` mutably to get the EventBus.
    
    if !events_to_send.is_empty() {
        let event_bus = resources.get_mut::<EventBus>()
            .expect("EventBus resource is missing");

        for event in events_to_send {
            event_bus.send(event);
        }
    }
}
