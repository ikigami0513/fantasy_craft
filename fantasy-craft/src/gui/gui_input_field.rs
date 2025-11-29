use hecs::Entity;
use macroquad::prelude::*;
use serde::Deserialize;
use crate::{gui::{font_component::FontComponent, gui_box::GuiBox, resources::UiResolvedRects}, prelude::{ColorData, ComponentLoader, Context, Vec2Data, Visible}};

#[derive(Debug, Clone)]
pub struct GuiInputField {
    pub text: String,
    pub is_focused: bool,
    pub caret_blink_timer: f32,
    pub caret_visible: bool,
    pub max_chars: Option<usize>,
    pub font_size: f32,
    pub color: Color,
    pub backspace_repeat_timer: f32,
    pub padding: Vec2,
    pub caret_position: usize,
    pub scroll_offset: f32,
    pub left_key_repeat_timer: f32,
    pub right_key_repeat_timer: f32
}

impl Default for GuiInputField {
    fn default() -> Self {
        Self {
            text: String::new(),
            is_focused: false,
            caret_blink_timer: 0.0,
            caret_visible: true,
            max_chars: None,
            font_size: 30.0,
            color: BLACK,
            backspace_repeat_timer: 0.0,
            padding: vec2(0.0, 0.0),
            caret_position: 0,
            scroll_offset: 0.0,
            left_key_repeat_timer: 0.0,
            right_key_repeat_timer: 0.0
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiInputFieldLoaderData {
    pub text: String,
    pub is_focused: bool,
    pub caret_blink_timer: f32,
    pub caret_visible: bool,
    pub max_chars: Option<usize>,
    pub font_size: f32,
    pub color: ColorData,
    pub backspace_repeat_timer: f32,
    pub padding: Vec2Data,
    pub caret_position: usize,
    pub scroll_offset: f32,
    pub left_key_repeat_timer: f32,
    pub right_key_repeat_timer: f32
}

pub struct GuiInputFieldLoader;

impl ComponentLoader for GuiInputFieldLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiInputFieldLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = GuiInputField {
            text: loader_data.text,
            is_focused: loader_data.is_focused,
            caret_blink_timer: loader_data.caret_blink_timer,
            caret_visible: loader_data.caret_visible,
            max_chars: loader_data.max_chars,
            font_size: loader_data.font_size,
            color: Color::new(
                loader_data.color.r,
                loader_data.color.g,
                loader_data.color.b,
                loader_data.color.a
            ),
            backspace_repeat_timer: loader_data.backspace_repeat_timer,
            padding: vec2(loader_data.padding.x, loader_data.padding.y),
            caret_position: loader_data.caret_position,
            scroll_offset: loader_data.scroll_offset,
            left_key_repeat_timer: loader_data.left_key_repeat_timer,
            right_key_repeat_timer: loader_data.right_key_repeat_timer
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiInputField");
    }
}

pub fn input_field_focus_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);

    if !is_pressed {
        return;
    }

    let mut clicked_entity: Option<Entity> = None;
    
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiBox, Option<&Visible>)>();

    for (entity, (gui_box, visibility)) in query.iter() {
        if ctx.world.get::<&GuiInputField>(entity).is_err() {
            continue;
        }

        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible || !gui_box.screen_space {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        if is_hovered {
            clicked_entity = Some(entity);
            break;
        }
    }

    // (Logic for setting focus is correct)
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

    // --- MODIFIED: Get dt once ---
    let dt = ctx.dt();
    
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&mut GuiInputField, &GuiBox, Option<&FontComponent>)>();

    for (entity, (input_field, _gui_box, font_opt)) in query.iter() {
        if !input_field.is_focused {
            input_field.backspace_repeat_timer = 0.0;
            input_field.left_key_repeat_timer = 0.0;
            input_field.right_key_repeat_timer = 0.0;
            continue;
        }
        
        // --- Left Arrow ---
        let left_pressed = is_key_pressed(KeyCode::Left);
        let left_down = is_key_down(KeyCode::Left);
        let mut move_left = false;

        if left_pressed {
            move_left = true;
            input_field.left_key_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        }
        else if left_down {
            // --- MODIFIED ---
            input_field.left_key_repeat_timer -= dt;
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

        // --- Right Arrow ---
        let right_pressed = is_key_pressed(KeyCode::Right);
        let right_down = is_key_down(KeyCode::Right);
        let mut move_right = false;

        if right_pressed {
            move_right = true;
            input_field.right_key_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        }
        else if right_down {
            // --- MODIFIED ---
            input_field.right_key_repeat_timer -= dt;
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

        // --- Backspace ---
        let backspace_pressed = is_key_pressed(KeyCode::Backspace);
        let backspace_down = is_key_down(KeyCode::Backspace);
        
        let mut should_delete = false;
        if backspace_pressed {
            should_delete = true;
            input_field.backspace_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        } else if backspace_down {
            // --- MODIFIED ---
            input_field.backspace_repeat_timer -= dt;
            if input_field.backspace_repeat_timer <= 0.0 {
                should_delete = true;
                input_field.backspace_repeat_timer = KEY_REPEAT_RATE;
            }
        } else {
            input_field.backspace_repeat_timer = 0.0;
        }

        if should_delete && input_field.caret_position > 0 {
            let mut chars: Vec<char> = input_field.text.chars().collect();
            if input_field.caret_position <= chars.len() {
                chars.remove(input_field.caret_position - 1);
                input_field.text = chars.into_iter().collect();
                input_field.caret_position -= 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }
        
        // --- Delete Key ---
        if is_key_pressed(KeyCode::Delete) {
             let mut chars: Vec<char> = input_field.text.chars().collect();
             if input_field.caret_position < chars.len() {
                chars.remove(input_field.caret_position);
                input_field.text = chars.into_iter().collect();
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
             }
        }

        // --- Typing ---
        while let Some(char) = get_char_pressed() {
            if char == '\u{08}' || char == '\u{7f}' { // Backspace or Delete
                continue; 
            }

            let char_count = input_field.text.chars().count();
            let at_limit = input_field.max_chars.map_or(false, |max| char_count >= max);
        
            if !at_limit {
                let mut chars: Vec<char> = input_field.text.chars().collect();
                let insert_pos = input_field.caret_position.min(chars.len());
                chars.insert(insert_pos, char);
                input_field.text = chars.into_iter().collect();
                
                input_field.caret_position += 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }


        // --- Scroll Logic ---
        let font_to_use: Option<&Font> = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));

        let text_before_caret: String = input_field.text.chars().take(input_field.caret_position).collect();
        let caret_x_absolute = measure_text(&text_before_caret, font_to_use, input_field.font_size as u16, 1.0).width;

        // --- MODIFIED ---
        let w = if let Some((_, size)) = resolved_rects_map.get(&entity) {
            size.x
        } else {
            300.0 // Fallback
        };

        let visible_width = w - (input_field.padding.x * 2.0);

        // (Scroll logic is correct)
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

        // --- Caret Blink ---
        // --- MODIFIED ---
        input_field.caret_blink_timer += dt;
        if input_field.caret_blink_timer >= 0.5 {
            input_field.caret_visible = !input_field.caret_visible;
            input_field.caret_blink_timer = 0.0;
        }
    }
}

pub fn input_field_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiInputField, &GuiBox, Option<&Visible>, Option<&FontComponent>)>();

    for (entity, (input_field, gui_box, visibility, font_opt)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible { continue; }

        if !gui_box.screen_space {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        let rt_w = (w.max(1.0)) as u32;
        let rt_h = (h.max(1.0)) as u32;
        let rt = render_target(rt_w, rt_h);

        let camera = Camera2D {
            render_target: Some(rt.clone()),
            viewport: None,
            zoom: vec2(2.0 / rt_w as f32, 2.0 / rt_h as f32),
            target: vec2(rt_w as f32 / 2.0, rt_h as f32 / 2.0),
            ..Default::default()
        };

        set_camera(&camera);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        let content_x = input_field.padding.x;
        let text_y_top = (rt_h as f32 - input_field.font_size) / 2.0;
        let baseline_y = text_y_top + input_field.font_size * 0.8; 
        let draw_x = content_x - input_field.scroll_offset;

        let font_to_use: Option<&Font> = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));

        // (Text drawing logic is correct)
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

        // (Caret drawing logic is correct)
        if input_field.is_focused && input_field.caret_visible {
            let text_before_caret: String = input_field.text.chars().take(input_field.caret_position).collect();
            let caret_offset = measure_text(&text_before_caret, font_to_use, input_field.font_size as u16, 1.0).width;
            let caret_x = draw_x + caret_offset;

            draw_rectangle(
                caret_x,
                text_y_top,
                2.0, 
                input_field.font_size,
                input_field.color
            );
        }

        set_default_camera();

        let draw_params = DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        };

        draw_texture_ex(&rt.texture, x, y, WHITE, draw_params);
    }
}
