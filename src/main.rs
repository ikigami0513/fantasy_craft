use hecs::World;
use macroquad::prelude::*;

mod assets;
mod asset_server;
mod camera;
mod components;
mod systems;
mod schedule;
mod context;
mod tiled_map;

use crate::asset_server::AssetServer;
use crate::camera::{update_camera, Camera, CameraTarget, MainCamera};
use crate::components::{AnimationComponent, Behavior, BehaviorComponent, DirectionComponent, NpcTag, PlayerTag, Speed, StateComponent, Transform, Velocity};
use crate::components::{Direction, State};
use crate::context::Context;
use crate::schedule::{Schedule, Stage};
use crate::tiled_map::{MainTileMap, TileMapComponent};
use systems::*;

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
            position: map_center,
            ..Default::default()
        },
        Camera {
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
        CameraTarget
    ));

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
        }
    ));
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut asset_server = AssetServer::new();

    asset_server.load_assets_from_file("resources/assets.json")
        .await
        .expect("Failed to load assets from JSON file");

    asset_server.load_tiled_map("test_map".to_string(), "resources/maps/tests.json")
        .await
        .expect("Failed to load Tiled map");

    let world = World::new();

    let mut context = Context {
        world,
        asset_server,
        dt: 0.0
    };

    let mut schedule = Schedule::new();

    schedule.add_system(Stage::StartUp, setup_system);
    
    schedule.add_system(Stage::Update, player_update);
    schedule.add_system(Stage::Update, npc_behavior_system);
    schedule.add_system(Stage::Update, movement_system);
    schedule.add_system(Stage::Update, update_animations);

    schedule.add_system(Stage::PostUpdate, update_camera);

    schedule.add_system(Stage::Render, render_system);

    let mut fps_timer: f32 = 0.0;
    let mut displayed_fps: i32 = get_fps();

    schedule.run_stage(Stage::StartUp, &mut context);

    loop {
        context.dt = get_frame_time();
        fps_timer += context.dt;

        if fps_timer >= 1.0 {
            displayed_fps = get_fps();
            fps_timer = 0.0;
        }

        clear_background(LIGHTGRAY);

        schedule.run_stage(Stage::Update, &mut context);
        schedule.run_stage(Stage::PostUpdate, &mut context);
        schedule.run_stage(Stage::Render, &mut context);

        let fps_text = format!("FPS: {}", displayed_fps);
        draw_text(&fps_text, 10.0, 25.0, 30.0, BLACK);

        next_frame().await
    }
}
