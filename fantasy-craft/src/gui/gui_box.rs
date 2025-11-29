use macroquad::prelude::*;
use serde::Deserialize;
use crate::{gui::{alignment::{HorizontalAlignment, HorizontalAlignmentType, VerticalAlignment, VerticalAlignmentType}, gui_button::{ButtonState, GuiButton}, gui_dimension::{GuiDimension, GuiDimensionLoaderData}, resources::UiResolvedRects}, prelude::{ColorData, ComponentLoader, Context, Visible}};

#[derive(Debug, Clone)]
pub struct GuiBox {
    pub width: GuiDimension,
    pub height: GuiDimension,
    pub color: Color,
    pub screen_space: bool,
    pub border_radius: f32,
}

impl Default for GuiBox {
    fn default() -> Self {
        Self {
            width: GuiDimension::Pixels(100.0),
            height: GuiDimension::Pixels(40.0),
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            screen_space: true,
            border_radius: 0.0
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiBoxLoaderData {
    #[serde(default)]
    pub width: GuiDimensionLoaderData,

    #[serde(default)]
    pub height: GuiDimensionLoaderData,
    
    pub color: ColorData,
    pub screen_space: bool,
    pub border_radius: f32
}

pub struct GuiBoxLoader;

impl ComponentLoader for GuiBoxLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiBoxLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let parse_dimension = |loader_dim: GuiDimensionLoaderData| -> GuiDimension {
            match loader_dim {
                GuiDimensionLoaderData::Pixels(px) => GuiDimension::Pixels(px),
                GuiDimensionLoaderData::Percent(s) => {
                    // Enlève le '%' et parse en f32
                    let value = s.trim_end_matches('%')
                                 .parse::<f32>()
                                 .unwrap_or(100.0); // 100% par défaut en cas d'erreur
                    
                    // Convertit en 0.0-1.0
                    GuiDimension::Percent(value / 100.0) 
                }
            }
        };

        let component = GuiBox {
            width: parse_dimension(loader_data.width),
            height: parse_dimension(loader_data.height),
            color: Color::new(
                loader_data.color.r,
                loader_data.color.g,
                loader_data.color.b,
                loader_data.color.a
            ),
            screen_space: loader_data.screen_space,
            border_radius: loader_data.border_radius
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiBox");
    }
}

pub fn gui_box_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(
        &GuiBox,
        Option<&GuiButton>,
        Option<&Visible>,
        Option<&HorizontalAlignment>,
        Option<&VerticalAlignment>,
    )>();

    for (entity, (gui_box, button_opt, visibility, h_align, v_align)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible || !gui_box.screen_space {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect // Dereference the tuple (pos, size)
            } else {
                continue; // This UI element was not processed by the layout system
            };

        let mut x = resolved_pos.x;
        let mut y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;
        // --- END OF CHANGE ---

        // (Alignment logic is correct)
        if let Some(h_align) = h_align {
            match h_align.0 {
                HorizontalAlignmentType::Left => { /* Default */ }
                HorizontalAlignmentType::Center => x -= w / 2.0,
                HorizontalAlignmentType::Right => x -= w,
            }
        }
        if let Some(v_align) = v_align {
            match v_align.0 {
                VerticalAlignmentType::Top => { /* Default */ }
                VerticalAlignmentType::Center => y -= h / 2.0,
                VerticalAlignmentType::Bottom => y -= h,
            }
        }
        
        let radius = gui_box.border_radius.min(w / 2.0).min(h / 2.0);

        // Determine the final color
        let mut final_color = gui_box.color;
        if let Some(button) = button_opt {
            final_color = match button.state {
                ButtonState::Hovered => button.hovered_color,
                ButtonState::Pressed => button.pressed_color,
                ButtonState::Idle => button.normal_color
            };
        }

        // (Drawing logic is correct)
        if radius == 0.0 {
            draw_rectangle(x, y, w, h, final_color);
        } else {
            // 1. Create an opaque version
            let opaque_color = Color::new(final_color.r, final_color.g, final_color.b, 1.0);

            // 2. Create the render target
            let rt_w = w.max(1.0) as u32;
            let rt_h = h.max(1.0) as u32;
            let rt = render_target(rt_w, rt_h);
            rt.texture.set_filter(FilterMode::Nearest);

            // 3. Set up a camera
            let camera = Camera2D {
                render_target: Some(rt.clone()),
                zoom: vec2(2.0 / rt_w as f32, 2.0 / rt_h as f32),
                target: vec2(rt_w as f32 / 2.0, rt_h as f32 / 2.0),
                ..Default::default()
            };
            set_camera(&camera);

            // 4. Draw the 7 shapes (OPAQUE) at (0, 0)
            clear_background(BLANK);
            draw_rectangle(radius, 0.0, w - radius * 2.0, h, opaque_color);
            draw_rectangle(0.0, radius, radius, h - radius * 2.0, opaque_color);
            draw_rectangle(w - radius, radius, radius, h - radius * 2.0, opaque_color);
            draw_circle(radius, radius, radius, opaque_color);
            draw_circle(w - radius, radius, radius, opaque_color);
            draw_circle(radius, h - radius, radius, opaque_color);
            draw_circle(w - radius, h - radius, radius, opaque_color);

            // 5. Restore the default camera
            set_default_camera();

            // 6. Draw the RenderTarget to the screen at its final, aligned position
            draw_texture_ex(
                &rt.texture,
                x, // Use the aligned x
                y, // Use the aligned y
                final_color, // Use the original color (with alpha)
                DrawTextureParams {
                    dest_size: Some(vec2(w, h)),
                    ..Default::default()
                },
            );
        }
    }
}
