use engine::prelude::*;

use crate::headers::{files_menu::FilesMenuPlugin, settings_menu::SettingsMenuPlugin};

pub struct HeadersPlugin;

impl Plugin for HeadersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(FilesMenuPlugin)
            .add_plugin(SettingsMenuPlugin);
    }
}