use hecs::World;
use macroquad::prelude::*;

mod assets;
mod asset_server;
mod camera;
mod components;
mod systems;
mod schedule;
mod context;

use crate::asset_server::AssetServer;
use crate::camera::Camera;
use crate::components::{AnimationComponent, Behavior, BehaviorComponent, DirectionComponent, NpcTag, PlayerTag, Speed, StateComponent, Transform, Velocity};
use crate::components::{Direction, State};
use crate::context::Context;
use crate::schedule::{Schedule, Stage};
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
    ctx.world.spawn((
        Transform::default(),
        Velocity(Vec2::ZERO),
        Speed(100.0),
        DirectionComponent(Direction::Down),
        StateComponent(State::Idle),
        AnimationComponent("player_base_idle_down".to_string()),
        PlayerTag
    ));

    ctx.world.spawn((
        Transform {
            position: Vec2::new(100.0, 100.0),
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

    schedule.add_system(Stage::Render, render_system);

    schedule.run_stage(Stage::StartUp, &mut context);

    let player_position = context.world.query::<&Transform>()
        .with::<&PlayerTag>()
        .iter()
        .next()
        .map(|(_, t)| t.position)
        .unwrap_or(Vec2::ZERO);

    let mut game_camera = Camera::new(player_position);
    game_camera.zoom = 2.0;

    loop {
        context.dt = get_frame_time();

        clear_background(LIGHTGRAY);

        schedule.run_stage(Stage::Update, &mut context);
        schedule.run_stage(Stage::PostUpdate, &mut context);

        if let Some((_, transform)) = context.world.query::<&Transform>().with::<&PlayerTag>().iter().next() {
            game_camera.update(transform.position, context.dt);
            game_camera.set_camera();
        }

        schedule.run_stage(Stage::Render, &mut context);

        game_camera.unset_camera();

        // draw gui here

        next_frame().await
    }
}
