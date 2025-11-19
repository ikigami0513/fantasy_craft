use crate::{audio::system::audio_system, prelude::{GameState, Plugin, Stage, System}};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.add_system(Stage::PostUpdate, System::new(
            audio_system,
            vec![GameState::Playing, GameState::Menu]
        ));
    }
}
