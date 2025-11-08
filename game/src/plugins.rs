use engine::prelude::*;

use crate::{components::{AnimationPrefixLoader, BehaviorComponentLoader, NpcTagLoader, PlayerTagLoader}, systems::{check_player_npc_collision, npc_behavior_system, player_update}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.scene_loader
            .register("BehaviorComponent", Box::new(BehaviorComponentLoader))
            .register("PlayerTag", Box::new(PlayerTagLoader))
            .register("AnimationPrefix", Box::new(AnimationPrefixLoader))
            .register("NpcTag", Box::new(NpcTagLoader));

        app
            .add_system(Stage::Update, player_update)
            .add_system(Stage::PostUpdate, check_player_npc_collision);
    }
}

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Stage::Update, npc_behavior_system);
    }
}
