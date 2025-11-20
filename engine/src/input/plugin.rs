use crate::{input::{focus::InputFocus, manager::InputManager, system::input_focus_update_system}, prelude::{App, GameState, Plugin, Stage, System}};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.context.insert_resource(InputFocus::default());
        app.context.insert_resource(InputManager::new());

        app.add_system(Stage::Update, System::new(
            input_focus_update_system,
            vec![GameState::Playing, GameState::Menu]
        ));
    }
}
