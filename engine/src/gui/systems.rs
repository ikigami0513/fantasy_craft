use macroquad::prelude::*;
use crate::core::context::Context;
use crate::gui::components::{TextDisplay, GuiBox};
use crate::physics::components::Transform;
use crate::prelude::{ButtonState, GuiButton};

pub fn button_interaction_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_down(MouseButton::Left);
    let just_clicked = is_mouse_button_pressed(MouseButton::Left);

    let mut query = ctx.world.query::<(&mut GuiButton, &Transform, &GuiBox)>();

    for (_, (button, transform, gui_box)) in query.iter() {
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
    for (_, (gui_box, transform, button_opt)) in ctx.world.query::<(&GuiBox, &Transform, Option<&GuiButton>)>().iter() {
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
    for (_, (text_display, transform)) in ctx.world.query::<(&TextDisplay, &Transform)>().iter() {
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
