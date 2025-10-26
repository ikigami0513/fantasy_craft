use macroquad::prelude::*;
use engine::prelude::*;

mod components;
mod systems;
mod plugins;

use crate::{components::{AnimationPrefix, Behavior, BehaviorComponent, ClickMeAction, FpsDisplay, NpcTag, PlayerTag}, plugins::{NpcPlugin, PlayerPlugin}, systems::{click_me_system, fps_display_update}};

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

    let button = ctx.world.spawn((
        Transform {
            position: vec2(screen_width() - 150.0, 10.0),
            ..Default::default()
        },
        GuiButton {
            normal_color: Color::new(0.20, 0.45, 0.85, 1.0),
            hovered_color: Color::new(0.25, 0.55, 0.95, 1.0),
            pressed_color: Color::new(0.15, 0.35, 0.75, 1.0),
            ..Default::default()
        },
        GuiBox {
            width: 120.0,
            height: 30.0,
            border_radius: 5.0,
            ..Default::default()
        },
        ClickMeAction
    ));

    ctx.world.spawn((
        Transform::default(),
        LocalOffset(vec2(10.0, -7.0)),
        TextDisplay {
            text: "Click me".to_string(),
            font_size: 30.0,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
        Parent(button)
    ));

    let fps_text_box = ctx.world.spawn((
        Transform {
            position: vec2(10.0, 10.0),
            ..Default::default()
        },
        GuiBox {
            width: 120.0,
            height: 30.0,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            screen_space: true,
            border_radius: 5.0
        }
    ));
    
    ctx.world.spawn((
        Transform::default(),
        FpsDisplay { fps_timer: 0.0, displayed_fps: get_fps() },
        TextDisplay {
            text: "FPS: 60".to_string(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
        Parent(fps_text_box),
        LocalOffset(vec2(5.0, -5.0))
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
    let player = ctx.world.spawn((
        Transform {
            position: map_center,
            ..Default::default()
        },
        Velocity(Vec2::ZERO),
        Speed(100.0),
        DirectionComponent(Direction::Down),
        StateComponent(State::Idle),
        PlayerTag,
        CameraTarget,
        RigidBody::new(BodyType::Dynamic),
        Collider::new_box(16.0, 20.0)
    ));

    ctx.world.spawn((
        Transform::default(),
        AnimationPrefix("player_base".to_string()),
        AnimationComponent("player_base_idle_down".to_string()),
        Parent(player),
        LocalOffset(Vec2::ZERO)
    ));

    ctx.world.spawn((
        Transform::default(),
        AnimationPrefix("player_hand".to_string()),
        AnimationComponent("player_hand_idle_down".to_string()),
        Parent(player),
        LocalOffset(Vec2::ZERO)
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
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_system(Stage::StartUp, setup_system)
        .add_system(Stage::Update, fps_display_update)
        .add_system(Stage::Update, click_me_system);

    app.run().await
}
