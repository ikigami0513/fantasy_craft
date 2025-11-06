use hecs::Entity;
use macroquad::prelude::*;
use crate::core::context::Context;
use crate::gui::components::{TextDisplay, GuiBox};
use crate::physics::components::Transform;
use crate::prelude::{ButtonState, FontComponent, GuiButton, GuiCheckbox, GuiDraggable, GuiImage, GuiInputField, GuiSlider, Visible};

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
    for (_, (text_display, transform, visibility, font_opt)) in ctx.world.query::<(&TextDisplay, &Transform, Option<&Visible>, Option<&FontComponent>)>().iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        if text_display.screen_space {
            let baseline_y = transform.position.y + text_display.font_size;

            if let Some(font) = font_opt {
                draw_text_ex(
                    &text_display.text,
                    transform.position.x,
                    baseline_y,
                    TextParams {
                        font: ctx.asset_server.get_font(&font.0),
                        font_size: text_display.font_size as u16,
                        color: text_display.color,
                        ..Default::default()
                    }
                );
            }
            else {
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
            if !input_field.is_focused {
                while get_char_pressed().is_some() {}
                input_field.caret_position = input_field.text.chars().count()
            }

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
    const KEY_REPEAT_INITIAL_DELAY: f32 = 0.4;
    const KEY_REPEAT_RATE: f32 = 0.05;

    for (_, (input_field, gui_box, font_opt)) in ctx.world.query::<(&mut GuiInputField, &GuiBox, Option<&FontComponent>)>().iter() {
        if !input_field.is_focused {
            input_field.backspace_repeat_timer = 0.0;
            input_field.left_key_repeat_timer = 0.0;
            input_field.right_key_repeat_timer = 0.0;
            continue;
        }

        // --- GESTION FLÈCHE GAUCHE (avec répétition) ---
        let left_pressed = is_key_pressed(KeyCode::Left);
        let left_down = is_key_down(KeyCode::Left);
        let mut move_left = false;

        if left_pressed {
            move_left = true;
            input_field.left_key_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        }
        else if left_down {
            input_field.left_key_repeat_timer -= ctx.dt;
            if input_field.left_key_repeat_timer <= 0.0 {
                move_left = true;
                input_field.left_key_repeat_timer = KEY_REPEAT_RATE;
            }
        }
        else {
            input_field.left_key_repeat_timer = 0.0;
        }

        if move_left && input_field.caret_position > 0 {
            input_field.caret_position -= 1;
            input_field.caret_visible = true;
            input_field.caret_blink_timer = 0.0;
        }

        // --- GESTION FLÈCHE DROITE (avec répétition) ---
        let right_pressed = is_key_pressed(KeyCode::Right);
        let right_down = is_key_down(KeyCode::Right);
        let mut move_right = false;

        if right_pressed {
            move_right = true;
            input_field.right_key_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        }
        else if right_down {
            input_field.right_key_repeat_timer -= ctx.dt;
            if input_field.right_key_repeat_timer <= 0.0 {
                move_right = true;
                input_field.right_key_repeat_timer = KEY_REPEAT_RATE;
            }
        }
        else {
            input_field.right_key_repeat_timer = 0.0;
        }

        if move_right {
            let text_len = input_field.text.chars().count();
            if input_field.caret_position < text_len {
                input_field.caret_position += 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }

        // --- GESTION DU BACKSPACE (Suppression) ---
        let backspace_pressed = is_key_pressed(KeyCode::Backspace);
        let backspace_down = is_key_down(KeyCode::Backspace);
        
        let mut should_delete = false;
        if backspace_pressed {
            should_delete = true;
            input_field.backspace_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        } else if backspace_down {
            input_field.backspace_repeat_timer -= ctx.dt;
            if input_field.backspace_repeat_timer <= 0.0 {
                should_delete = true;
                input_field.backspace_repeat_timer = KEY_REPEAT_RATE;
            }
        } else {
            input_field.backspace_repeat_timer = 0.0;
        }

        if should_delete && input_field.caret_position > 0 {
            // Suppression sécurisée UTF-8
            let mut chars: Vec<char> = input_field.text.chars().collect();
            if input_field.caret_position <= chars.len() {
                chars.remove(input_field.caret_position - 1);
                input_field.text = chars.into_iter().collect();
                input_field.caret_position -= 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }
        
        // GESTION SUPPR (Delete) - Optionnel mais pratique
        if is_key_pressed(KeyCode::Delete) {
             let mut chars: Vec<char> = input_field.text.chars().collect();
             if input_field.caret_position < chars.len() {
                chars.remove(input_field.caret_position);
                input_field.text = chars.into_iter().collect();
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
             }
        }

        // --- GESTION DE LA SAISIE (Insertion) ---
        while let Some(char) = get_char_pressed() {
            if char == '\u{08}' || char == '\u{7f}' { // Backspace ou Delete (par sécurité)
                continue; 
            }

            let char_count = input_field.text.chars().count();
            let at_limit = input_field.max_chars.map_or(false, |max| char_count >= max);
        
            if !at_limit {
                // Insertion sécurisée UTF-8 à la position du curseur
                let mut chars: Vec<char> = input_field.text.chars().collect();
                // Sécurité : s'assurer que la position est valide
                let insert_pos = input_field.caret_position.min(chars.len());
                chars.insert(insert_pos, char);
                input_field.text = chars.into_iter().collect();
                
                input_field.caret_position += 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }

        let font_to_use: Option<&Font> = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));

        let text_before_caret: String = input_field.text.chars().take(input_field.caret_position).collect();
        let caret_x_absolute = measure_text(&text_before_caret, font_to_use, input_field.font_size as u16, 1.0).width;

        let visible_width = gui_box.width - (input_field.padding.x * 2.0);

        if caret_x_absolute < input_field.scroll_offset {
            input_field.scroll_offset = caret_x_absolute;
        }

        if caret_x_absolute > input_field.scroll_offset + visible_width {
            input_field.scroll_offset = caret_x_absolute - visible_width;
        }

        let total_text_width = measure_text(&input_field.text, font_to_use, input_field.font_size as u16, 1.0).width;
        if total_text_width < visible_width {
             input_field.scroll_offset = 0.0;
        } else if total_text_width - input_field.scroll_offset < visible_width {
             input_field.scroll_offset = (total_text_width - visible_width).max(0.0);
        }

        input_field.caret_blink_timer += ctx.dt;
        if input_field.caret_blink_timer >= 0.5 {
            input_field.caret_visible = !input_field.caret_visible;
            input_field.caret_blink_timer = 0.0;
        }
    }
}

pub fn input_field_render_system(ctx: &mut Context) {
    let mut query = ctx.world.query::<(&GuiInputField, &Transform, &GuiBox, Option<&Visible>, Option<&FontComponent>)>();

    for (_, (input_field, transform, gui_box, visibility, font_opt)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible { continue; }

        if !gui_box.screen_space {
            // This example only handles screen-space fields.
            continue;
        }

        // Ensure integer, non-zero render target dimensions
        let rt_w = (gui_box.width.max(1.0)) as u32;
        let rt_h = (gui_box.height.max(1.0)) as u32;

        // --- Create render target (temporary) ---
        // For production: cache render targets in `ctx` keyed by (width,height) or entity.
        let rt = render_target(rt_w, rt_h);

        // --- Set up a Camera2D that renders into the RenderTarget ---
        let camera = Camera2D {
            render_target: Some(rt.clone()),
            // we define the visible region to match the render target's size
            // origin at (0, 0) top-left, with height inverted for standard coordinates
            // (Macroquad y-axis goes down by default)
            viewport: None,
            zoom: vec2(2.0 / rt_w as f32, 2.0 / rt_h as f32),
            target: vec2(rt_w as f32 / 2.0, rt_h as f32 / 2.0),
            ..Default::default()
        };

        // Draw into the render target
        // Set the camera associated with the render target so that coordinates are local to [0..rt_w] x [0..rt_h]
        set_camera(&camera);

        // Clear with transparent background
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        // Coordinates inside the render target:
        // content_x is the left edge inside the RT where text starts (padding)
        let content_x = input_field.padding.x;
        // vertical centering approximation inside RT
        let text_y_top = (rt_h as f32 - input_field.font_size) / 2.0;
        let baseline_y = text_y_top + input_field.font_size * 0.8; // visual baseline estimate

        // Adjust draw_x by the scroll offset (scrolling moves the text left)
        let draw_x = content_x - input_field.scroll_offset;

        // Choose font if any
        let font_to_use: Option<&Font> = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));

        // Draw the input text inside the RT
        if let Some(font) = font_to_use {
            draw_text_ex(
                &input_field.text,
                draw_x,
                baseline_y,
                TextParams {
                    font: Some(font),
                    font_size: input_field.font_size as u16,
                    color: input_field.color,
                    ..Default::default()
                }
            );
        } else {
            draw_text(
                &input_field.text,
                draw_x,
                baseline_y,
                input_field.font_size,
                input_field.color
            );
        }

        // Draw caret inside the RT if focused
        if input_field.is_focused && input_field.caret_visible {
            let text_before_caret: String = input_field.text.chars().take(input_field.caret_position).collect();
            let caret_offset = measure_text(&text_before_caret, font_to_use, input_field.font_size as u16, 1.0).width;
            let caret_x = draw_x + caret_offset;

            draw_rectangle(
                caret_x,
                text_y_top,
                2.0, // cursor width
                input_field.font_size,
                input_field.color
            );
        }

        // Done drawing to RT: restore default camera (the screen)
        set_default_camera();

        // Draw the RenderTarget texture onto the screen at the GUI box position.
        // We use dest_size to keep the same logical size as the GUI box.
        let draw_params = DrawTextureParams {
            dest_size: Some(vec2(gui_box.width, gui_box.height)),
            ..Default::default()
        };

        draw_texture_ex(&rt.texture, transform.position.x, transform.position.y, WHITE, draw_params);

        // NOTE:
        // `rt` will be dropped here at end of scope. If you cache RTs, don't drop them.
    }
}

pub fn input_focus_update_system(ctx: &mut Context) {
    ctx.input_focus.is_captured_by_ui = false;

    for (_, input_field) in ctx.world.query::<&GuiInputField>().iter() {
        if input_field.is_focused {
            ctx.input_focus.is_captured_by_ui = true;
            break;
        }
    }
}

pub fn gui_image_render_system(ctx: &mut Context) {
    let mut query = ctx.world.query::<(&GuiImage, &Transform, Option<&GuiBox>, Option<&Visible>)>();

    for (_, (gui_image, transform, gui_box_opt, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible {
            continue;
        }

        if !gui_image.screen_space {
            continue;
        }

        if let Some(spritesheet_name) = &gui_image.texture {
            if let Some(spritesheet) = ctx.asset_server.get_spritesheet(&spritesheet_name) {
                let texture = &spritesheet.texture;
                let source = spritesheet.get_source_rect(gui_image.col_row.x, gui_image.col_row.y);

                let dest_size = if let Some(gui_box) = gui_box_opt {
                    vec2(gui_box.width, gui_box.height)
                } else {
                    vec2(spritesheet.sprite_width * transform.scale.x, spritesheet.sprite_height * transform.scale.y)
                };

                let draw_x = transform.position.x;
                let draw_y = transform.position.y;

                let draw_params = DrawTextureParams {
                    dest_size: Some(dest_size),
                    source,
                    ..Default::default()
                };

                draw_texture_ex(texture, draw_x, draw_y, gui_image.tint, draw_params);
            }
        }
    }
}
