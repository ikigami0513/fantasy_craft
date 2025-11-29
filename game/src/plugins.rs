use fantasy_craft::prelude::*;

use crate::{components::{AnimationPrefixLoader, BehaviorComponentLoader, MainMenuLoader, NpcTagLoader, PlayerTagLoader}, systems::{check_player_npc_collision, npc_behavior_system, player_update, menu_buttons_system, toggle_main_menu_system}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.scene_loader
            .register("BehaviorComponent", Box::new(BehaviorComponentLoader))
            .register("PlayerTag", Box::new(PlayerTagLoader))
            .register("AnimationPrefix", Box::new(AnimationPrefixLoader))
            .register("NpcTag", Box::new(NpcTagLoader))
            .register("MainMenu", Box::new(MainMenuLoader));

        app
            .add_system(Stage::Update, System::new(
                player_update,
                vec![GameState::Playing]
            ))
            .add_system(Stage::Update, System::new(
                menu_buttons_system,
                vec![GameState::Menu]
            ))
            .add_system(Stage::Update, System::new(
                toggle_main_menu_system,
                vec![GameState::Playing, GameState::Menu]
            ))
            .add_system(Stage::PostUpdate, System::new(
                check_player_npc_collision,
                vec![GameState::Playing]
            ));
    }
}

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Stage::Update, System::new(
                npc_behavior_system,
                vec![GameState::Playing]
            ));
    }
}
