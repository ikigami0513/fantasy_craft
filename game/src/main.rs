use macroquad::prelude::*;
use engine::prelude::*;

mod components;
mod systems;
mod plugins;

use crate::{components::{Behavior, BehaviorComponent, FpsDisplay, NpcTag, PlayerTag}, plugins::{NpcPlugin, PlayerPlugin}, systems::fps_display_update};

fn window_conf() -> Conf {
    Conf {
        window_title: "Fantasy Craft".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

pub fn setup_system(ctx: &mut Context) {
    let map = ctx.asset_server.get_map("test_map").unwrap();
    let map_center = Vec2::new((map.width as f32 * map.tile_width as f32) / 2.0, (map.height as f32 * map.tile_height as f32) / 2.0);

    ctx.world.spawn((
        Transform {
            position: vec2(10.0, 25.0),
            ..Default::default()
        },
        FpsDisplay { fps_timer: 0.0, displayed_fps: get_fps() },
        TextDisplay {
            text: "FPS: 60".to_string(),
            ..Default::default()
        }
    ));

    ctx.world.spawn((
        Transform {
            position: map_center,
            ..Default::default()
        },
        CameraComponent {
            lerp_factor: 8.0,
            zoom: 1.5
        },
        MainCamera
    ));

    ctx.world.spawn((
        TileMapComponent{
            name: "test_map".to_string()
        },
        MainTileMap
    ));

    // Player
    ctx.world.spawn((
        Transform {
            position: map_center,
            ..Default::default()
        },
        Velocity(Vec2::ZERO),
        Speed(100.0),
        DirectionComponent(Direction::Down),
        StateComponent(State::Idle),
        AnimationComponent("player_base_idle_down".to_string()),
        PlayerTag,
        CameraTarget,
        RigidBody::new(BodyType::Dynamic),
        Collider::new_box(16.0, 20.0)
    ));

    // Farmer Npc
    ctx.world.spawn((
        Transform {
            position: Vec2::new(map_center.x - 50.0, map_center.y - 50.0),
            ..Default::default()
        },
        Speed(100.0),
        DirectionComponent(Direction::Down),
        StateComponent(State::Idle),
        AnimationComponent("farmer_idle_down".to_string()),
        BehaviorComponent(Behavior::Wander),
        NpcTag {
            name: "farmer".to_string(),
            wander_time: 0.0,
            wander_target_duration: 0.0
        },
        RigidBody::new(BodyType::Dynamic),
        Collider::new_box(16.0, 20.0)
    ));
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new(window_conf());

    app
        .add_plugin(Default2dPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_system(Stage::StartUp, setup_system)
        .add_system(Stage::Update, fps_display_update);

    app.run().await
}
