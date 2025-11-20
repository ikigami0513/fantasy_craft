use crate::{gui::gui_input_field::GuiInputField, input::focus::InputFocus, prelude::Context};

pub fn input_focus_update_system(ctx: &mut Context) {
    // --- MODIFIED ---
    // Get the resource mutably once.
    let (_world, resources) = (&ctx.world, &mut ctx.resources);

    let input_focus = resources.get_mut::<InputFocus>()
        .expect("Ressource InputFocus manquante");
    input_focus.is_captured_by_ui = false;

    for (_, input_field) in ctx.world.query::<&GuiInputField>().iter() {
        if input_field.is_focused {
            // --- MODIFIED ---
            input_focus.is_captured_by_ui = true;
            break;
        }
    }
}
