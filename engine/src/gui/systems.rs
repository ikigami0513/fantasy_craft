use hecs::Entity;
use macroquad::prelude::*;
use crate::core::context::Context;
use crate::gui::components::{TextDisplay, GuiBox};
use crate::physics::components::Transform;
use crate::prelude::{ButtonState, GuiButton, GuiCheckbox, GuiDraggable, GuiInputField, GuiSlider, Visible};

pub fn button_interaction_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_down(MouseButton::Left);
    let just_clicked = is_mouse_button_pressed(MouseButton::Left);

    let mut query = ctx.world.query::<(&mut GuiButton, &Transform, &GuiBox, Option<&Visible>)>();

    for (_, (button, transform, gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        button.just_clicked = false;

        if !gui_box.screen_space { continue; }

        let x = transform.position.x;
        let y = transform.position.y;
        let w = gui_box.width;
        let h = gui_box.height;

        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        match button.state {
            ButtonState::Idle => {
                if is_hovered {
                    button.state = ButtonState::Hovered;
                }
            }
            ButtonState::Hovered => {
                if !is_hovered {
                    button.state = ButtonState::Idle;
                }
                else if just_clicked {
                    button.state = ButtonState::Pressed;
                }
            }
            ButtonState::Pressed => {
                if !is_pressed {
                    if is_hovered {
                        button.just_clicked = true;
                        button.state = ButtonState::Hovered;
                    }
                    else {
                        button.state = ButtonState::Idle;
                    }
                }
            }
        }
    }
}

pub fn gui_box_render_system(ctx: &mut Context) {
    for (_, (gui_box, transform, button_opt, visibility)) in ctx.world.query::<(&GuiBox, &Transform, Option<&GuiButton>, Option<&Visible>)>().iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        if !gui_box.screen_space {
            continue;
        }

        let x = transform.position.x;
        let y = transform.position.y;
        let w = gui_box.width;
        let h = gui_box.height;
        let radius = gui_box.border_radius.min(w / 2.0).min(h / 2.0);

        let mut color = gui_box.color;
        if let Some(button) = button_opt {
            color = match button.state {
                ButtonState::Hovered => button.hovered_color,
                ButtonState::Pressed => button.pressed_color,
                ButtonState::Idle => button.normal_color
            };
        }

        if radius == 0.0 {
            // Si pas d'arrondi, on utilise la fonction rapide
            draw_rectangle(x, y, w, h, color);
        } else {
            // Logique de dessin manuel pour les coins arrondis :
            
            // 1. Dessiner la partie centrale (verticale)
            draw_rectangle(x + radius, y, w - radius * 2.0, h, color);
            
            // 2. Dessiner la partie gauche (sans les coins)
            draw_rectangle(x, y + radius, radius, h - radius * 2.0, color);
            
            // 3. Dessiner la partie droite (sans les coins)
            draw_rectangle(x + w - radius, y + radius, radius, h - radius * 2.0, color);

            // 4. Dessiner les 4 coins (cercles)
            draw_circle(x + radius, y + radius, radius, color); // Haut-Gauche
            draw_circle(x + w - radius, y + radius, radius, color); // Haut-Droit
            draw_circle(x + radius, y + h - radius, radius, color); // Bas-Gauche
            draw_circle(x + w - radius, y + h - radius, radius, color); // Bas-Droit
        }
    }
}

pub fn text_render_system(ctx: &mut Context) {
    for (_, (text_display, transform, visibility)) in ctx.world.query::<(&TextDisplay, &Transform, Option<&Visible>)>().iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        if text_display.screen_space {
            let baseline_y = transform.position.y + text_display.font_size;

            draw_text(
                &text_display.text,
                transform.position.x,
                baseline_y,
                text_display.font_size,
                text_display.color
            );
        }
    }
}

pub fn draggable_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();

    let current_mouse_pos = vec2(mouse_x, mouse_y);
    let delta = current_mouse_pos - ctx.prev_mouse_pos;
    
    let is_pressed = is_mouse_button_pressed(MouseButton::Left); // Pour commencer le drag
    let is_down = is_mouse_button_down(MouseButton::Left);     // Pour maintenir/arrêter le drag

    let mut query = ctx.world.query::<(&mut GuiDraggable, &mut Transform, &GuiBox, Option<&Visible>)>();

    for (_, (draggable, transform, gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        if draggable.is_dragging {
            // Si on est en train de glisser
            if !is_down { // On vérifie si le bouton est RELÂCHÉ (n'est plus enfoncé)
                draggable.is_dragging = false;
            } else {
                // Le bouton est toujours enfoncé, on continue de bouger
                transform.position.x += delta.x;
                transform.position.y += delta.y;
            }
        } else {
            // On ne glisse pas, on vérifie si on doit commencer
            let x = transform.position.x;
            let y = transform.position.y;
            let w = gui_box.width;
            let h = gui_box.height;

            let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

            if is_hovered && is_pressed { // On commence sur le clic initial
                draggable.is_dragging = true;
            }
        }
    }
}

pub fn slider_interaction_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);
    let is_down = is_mouse_button_down(MouseButton::Left);

    let mut query = ctx.world.query::<(&mut GuiSlider, &Transform, &GuiBox, Option<&Visible>)>();

    for (_, (slider, transform, gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        let x = transform.position.x;
        let y = transform.position.y;
        let w = gui_box.width;
        let h = gui_box.height;
        
        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        if slider.is_dragging_handle {
            if !is_down {
                slider.is_dragging_handle = false;
            } else {
                // On continue de glisser, on met à jour la valeur
                let relative_x = mouse_x - x;
                let normalized_value = (relative_x / w).clamp(0.0, 1.0);
                slider.value = slider.min + normalized_value * (slider.max - slider.min);
            }
        } else if is_hovered && is_pressed {
            // On commence à glisser (et on met à jour la valeur immédiatement)
            slider.is_dragging_handle = true;
            let relative_x = mouse_x - x;
            let normalized_value = (relative_x / w).clamp(0.0, 1.0);
            slider.value = slider.min + normalized_value * (slider.max - slider.min);
        }
    }
}

pub fn slider_render_system(ctx: &mut Context) {
    let mut query = ctx.world.query::<(&GuiSlider, &Transform, &GuiBox, Option<&Visible>)>();

    for (_, (slider, transform, gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        let normalized_value = (slider.value - slider.min) / (slider.max - slider.min).max(f32::EPSILON);
        let handle_width = slider.handle_width;
        let handle_x = transform.position.x + (normalized_value * gui_box.width) - (handle_width / 2.0);

        draw_rectangle(
            handle_x.clamp(transform.position.x, transform.position.x + gui_box.width - handle_width),
            transform.position.y,
            handle_width,
            gui_box.height,
            slider.handle_color
        )
    }
}

pub fn checkbox_logic_system(ctx: &mut Context) {
    let mut query = ctx.world.query::<(&GuiButton, &mut GuiCheckbox, Option<&Visible>)>();

    for (_, (button, checkbox, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }
        
        if button.just_clicked {
            checkbox.is_checked = !checkbox.is_checked;
        }
    }
}

pub fn checkbox_render_system(ctx: &mut Context) {
    let mut query = ctx.world.query::<(&GuiCheckbox, &Transform, &GuiBox, Option<&Visible>)>();

    for (_, (checkbox, transform, gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        if checkbox.is_checked {
            let x = transform.position.x;
            let y = transform.position.y;
            let w = gui_box.width;
            let h = gui_box.height;

            let padding = w * 0.2;
            draw_line(x + padding, y + padding, x + w - padding, y + h - padding, 2.0, BLACK);
            draw_line(x + w - padding, y + padding, x + padding, y + h - padding, 2.0, BLACK);
        }
    }
}

pub fn input_field_focus_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);

    if !is_pressed {
        return;
    }

    let mut clicked_entity: Option<Entity> = None;
    
    let mut query = ctx.world.query::<(&Transform, &GuiBox, Option<&Visible>)>();

    for (e, (transform, gui_box, visibility)) in query.iter() {
        if ctx.world.get::<&GuiInputField>(e).is_err() {
            continue;
        }

        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible || !gui_box.screen_space {
            continue;
        }

        let x = transform.position.x;
        let y = transform.position.y;
        let w = gui_box.width;
        let h = gui_box.height;

        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        if is_hovered {
            clicked_entity = Some(e);
            break;
        }
    }

    let mut query = ctx.world.query::<&mut GuiInputField>();
    for (e, input_field) in query.iter() {
        if Some(e) == clicked_entity {
            input_field.is_focused = true;
            input_field.caret_visible = true;
            input_field.caret_blink_timer = 0.0;
        }
        else {
            input_field.is_focused = false;
        }
    }
}

pub fn input_field_typing_system(ctx: &mut Context) {
    const BACKSPACE_INITIAL_DELAY: f32 = 0.4;
    const BACKSPACE_REPEAT_RATE: f32 = 0.05;

    for (_, input_field) in ctx.world.query::<&mut GuiInputField>().iter() {
        if !input_field.is_focused {
            input_field.backspace_repeat_timer = 0.0;
            continue;
        }
        
        let backspace_pressed = is_key_pressed(KeyCode::Backspace);
        let backspace_down = is_key_down(KeyCode::Backspace);

        if backspace_pressed {
            input_field.text.pop();
            input_field.backspace_repeat_timer = BACKSPACE_INITIAL_DELAY;
            input_field.caret_visible = true;
            input_field.caret_blink_timer = 0.0;
        } else if backspace_down {
            input_field.backspace_repeat_timer -= ctx.dt;

            if input_field.backspace_repeat_timer <= 0.0 {
                input_field.text.pop();
                input_field.backspace_repeat_timer = BACKSPACE_REPEAT_RATE;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        } else {
            input_field.backspace_repeat_timer = 0.0;
        }

        let mut char_typed = false;
        while let Some(char) = get_char_pressed() {
            if char == '\u{08}' { 
                continue; 
            }

            let at_limit = input_field.max_chars.map_or(false, |max| input_field.text.len() >= max);
        
            if !at_limit {
                input_field.text.push(char);
                char_typed = true;
            }
        }

        if char_typed {
            input_field.caret_visible = true;
            input_field.caret_blink_timer = 0.0;
        }

        input_field.caret_blink_timer += ctx.dt;
        if input_field.caret_blink_timer >= 0.5 {
            input_field.caret_visible = !input_field.caret_visible;
            input_field.caret_blink_timer = 0.0;
        }
    }
}

pub fn input_field_render_system(ctx: &mut Context) {
    let mut query = ctx.world.query::<(&GuiInputField, &Transform, &GuiBox, Option<&Visible>)>();

    for (_, (input_field, transform, gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible {
            continue;
        }

        let padding = 5.0;
        let text_x = transform.position.x + padding;

        let text_y_top = transform.position.y + (gui_box.height / 2.0) - (input_field.font_size / 2.0);
    
        let baseline_y = text_y_top + input_field.font_size;

        draw_text(
            &input_field.text,
            text_x,
            baseline_y,
            input_field.font_size,
            input_field.color
        );

        if input_field.is_focused && input_field.caret_visible {
            let text_dims = measure_text(&input_field.text, None, input_field.font_size as u16, 1.0);
            
            let caret_x = text_x + text_dims.width;
            let caret_width = 2.0;

            draw_rectangle(
                caret_x,
                text_y_top,
                caret_width,
                input_field.font_size,
                input_field.color
            );
        }
    }
}
