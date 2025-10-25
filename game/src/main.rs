use hecs::World;
use macroquad::prelude::*;

use engine::asset_server::AssetServer;
use engine::camera::{update_camera, Camera, CameraTarget, MainCamera};
use engine::components::{AnimationComponent, Behavior, BehaviorComponent, DirectionComponent, NpcTag, PlayerTag, Speed, StateComponent, Transform, Velocity};
use engine::components::{Direction, State};
use engine::context::Context;
use engine::physics::{collider_debug_render_system, physics_system, BodyType, Collider, RigidBody};
use engine::schedule::{Schedule, Stage};
use engine::tiled_map::{MainTileMap, TileMapComponent};
use engine::systems::*;

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
        Collider::new_box(32.0, 32.0)
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
        Collider::new_box(32.0, 32.0)
    ));
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut asset_server = AssetServer::new();

    asset_server.load_assets_from_file("resources/assets.json")
        .await
        .expect("Failed to load assets from JSON file");

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

    schedule.add_system(Stage::PostUpdate, physics_system);
    schedule.add_system(Stage::PostUpdate, update_camera);

    schedule.add_system(Stage::Render, tiled_map_render_system);
    schedule.add_system(Stage::Render, entities_render_system);
    schedule.add_system(Stage::Render, collider_debug_render_system);

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

        // unset camera to draw gui
        set_default_camera();

        let fps_text = format!("FPS: {}", displayed_fps);
        draw_text(&fps_text, 10.0, 25.0, 30.0, BLACK);

        next_frame().await
    }
}
