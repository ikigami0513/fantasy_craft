use hecs::Entity;
use macroquad::prelude::*;
use engine::{gui::components::TextDisplay, prelude::*};
use ::rand::{seq::IteratorRandom, thread_rng, Rng};
use crate::components::{AnimationPrefix, Behavior, BehaviorComponent, ClickMeAction, FpsDisplay, NpcTag, PlayerTag};

pub fn setup_ui(ctx: &mut Context) {
    let debug_panel = ctx.world.spawn((
        Transform {
            position: vec2(10.0, 10.0),
            ..Default::default()
        },
        GuiBox {
            width: 140.0,
            height: 80.0,
            color: Color::new(0.5, 0.5, 0.5, 1.0),
            border_radius: 10.0,
            ..Default::default()
        },
        GuiDraggable {
            is_dragging: false
        },
        Visible(true)
    ));

    ctx.world.spawn((
        Transform::default(),
        TextDisplay {
            text: "Debug Menu".to_string(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
        Parent(debug_panel),
        LocalOffset(vec2(5.0, -5.0))
    ));

    ctx.world.spawn((
        Transform::default(),
        FpsDisplay { fps_timer: 0.0, displayed_fps: get_fps() },
        TextDisplay {
            text: "FPS: 60".to_string(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
        Parent(debug_panel),
        LocalOffset(vec2(5.0, 20.0))
    ));

    ctx.world.spawn(SliderBundle {
        transform: Transform {
            position: vec2(150.0, 300.0),
            ..Default::default()
        },
        gui_box: GuiBox {
            width: 200.0,
            height: 20.0,
            color: Color::new(0.3, 0.3, 0.3, 1.0),
            border_radius: 10.0,
            ..Default::default()
        },
        slider: GuiSlider {
            value: 0.5,
            min: 0.0,
            max: 1.0,
            is_dragging_handle: false,
            handle_color: LIGHTGRAY,
            handle_width: 10.0
        }
    });

    // Checkbox
    ctx.world.spawn(CheckboxBundle {
        transform: Transform {
            position: vec2(150.0, 350.0),
            ..Default::default()
        },
        ..Default::default()
    });

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

    ctx.world.spawn((
        Transform {
            position: vec2(150.0, 400.0), // Positionnez-le où vous voulez
            ..Default::default()
        },
        // Le fond visuel
        GuiBox {
            width: 200.0,
            height: 40.0,
            color: Color::new(0.9, 0.9, 0.9, 1.0), // Un gris clair
            border_radius: 5.0,
            ..Default::default()
        },
        // Le composant de logique de saisie
        GuiInputField {
            text: "".to_string(),
            font_size: 28.0,
            color: BLACK,
            padding: vec2(15.0, -2.0),
            ..Default::default()
        }
    ));
}

pub fn setup_system(ctx: &mut Context) {
    let map = ctx.asset_server.get_map("test_map").unwrap();
    let map_center = Vec2::new((map.width as f32 * map.tile_width as f32) / 2.0, (map.height as f32 * map.tile_height as f32) / 2.0);

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
        TileMapLayerComponent{
            tilemap_name: "test_map".to_string(),
            layer_name: "background".to_string()
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

pub fn npc_behavior_system(ctx: &mut Context) {
    for (_, (transform, npc, behavior, state, direction, speed, animation_comp)) in ctx.world.query::<(&mut Transform, &mut NpcTag, &BehaviorComponent, &mut StateComponent, &mut DirectionComponent, &Speed, &mut AnimationComponent)>().iter() {
        match behavior.0 {
            Behavior::Stand => {
                state.0 = State::Idle;
            },
            Behavior::Wander => {
                npc.wander_time += ctx.dt;
                let mut rng = thread_rng();

                if npc.wander_time >= npc.wander_target_duration {
                    npc.wander_time = 0.0;

                    if state.0 == State::Idle {
                        state.0 = State::Walk;
                        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
                        direction.0 = *directions.iter().choose(&mut rng).unwrap_or(&Direction::Down);
                        npc.wander_target_duration = rng.gen_range(1.0..=3.0);
                    }
                    else {
                        state.0 = State::Idle;
                        npc.wander_target_duration = rng.gen_range(2.0..=5.0);
                    }
                }

                if state.0 == State::Walk {
                    let direction_vec = match direction.0 {
                        Direction::Up => vec2(0.0, -1.0),
                        Direction::Down => vec2(0.0, 1.0),
                        Direction::Left => vec2(-1.0, 0.0),
                        Direction::Right => vec2(1.0, 0.0)
                    };
                    transform.position += direction_vec * speed.0 * ctx.dt;
                }
            }
        }

        animation_comp.0 = format!("{}_{}_{}", npc.name, state.0.to_str(), direction.0.to_str());
    }
}

pub fn player_update(ctx: &mut Context) {
    let input_is_captured = ctx.input_focus.is_captured_by_ui;

    let player_entities: Vec<Entity> = ctx.world.query::<&PlayerTag>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    for entity in player_entities {
        let (velocity, state, direction) = 
            if let Ok(data) = ctx.world.query_one_mut::<(&mut Velocity, &mut StateComponent, &mut DirectionComponent)>(entity) {
                data
            } else {
                continue;
            };

        velocity.0 = Vec2::ZERO;
        let mut moving = false;
        
        if !input_is_captured {
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { 
                velocity.0.x = 1.0; 
                direction.0 = Direction::Right;
                moving = true;
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::Q) { 
                velocity.0.x = -1.0; 
                direction.0 = Direction::Left;
                moving = true;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::Z) { 
                velocity.0.y = -1.0; 
                if !is_key_down(KeyCode::Right) && !is_key_down(KeyCode::Left) {
                    direction.0 = Direction::Up;
                }
                moving = true;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) { 
                velocity.0.y = 1.0; 
                if !is_key_down(KeyCode::Right) && !is_key_down(KeyCode::Left) {
                    direction.0 = Direction::Down;
                }
                moving = true;
            }
        }
        
        state.0 = if moving { State::Walk } else { State::Idle };

        let current_state = state.0;
        let current_direction = direction.0;

        let children = find_children(&ctx.world, entity);

        for child in children {
            if let Ok((animation_prefix, animation_comp)) = ctx.world.query_one_mut::<(&AnimationPrefix, &mut AnimationComponent)>(child) {
                animation_comp.0 = format!("{}_{}_{}", animation_prefix.0, current_state.to_str(), current_direction.to_str());
            }
        }
    }
}

pub fn fps_display_update(ctx: &mut Context) {
    for (_, (fps_display, text_display)) in ctx.world.query::<(&mut FpsDisplay, &mut TextDisplay)>().iter() {
        fps_display.fps_timer += ctx.dt;

        if fps_display.fps_timer >= 1.0 {
            fps_display.displayed_fps = get_fps();
            fps_display.fps_timer = 0.0;
            text_display.text = format!("FPS: {}", fps_display.displayed_fps);
        }
    }
}

pub fn check_player_npc_collision(ctx: &mut Context) {
    for event in ctx.collision_events.iter() {
        let e_a = event.entity_a;
        let e_b = event.entity_b;

        let a_is_player = ctx.world.get::<&PlayerTag>(e_a).is_ok();
        let b_is_player = ctx.world.get::<&PlayerTag>(e_b).is_ok();

        let a_is_npc = ctx.world.get::<&NpcTag>(e_a).is_ok();
        let b_is_npc = ctx.world.get::<&NpcTag>(e_b).is_ok();

        if a_is_player && b_is_npc {
            println!("💥 Collision détectée ! Joueur ({:?}) a touché PNJ ({:?})", e_a, e_b);
        }
        else if b_is_player && a_is_npc {
            println!("💥 Collision détectée ! PNJ ({:?}) a touché Joueur ({:?})", e_a, e_b);
        }
    }
}

pub fn click_me_system(ctx: &mut Context) {
    let mut query = ctx.world.query::<(&mut GuiButton, &ClickMeAction)>();

    for (_, (button, _action)) in query.iter() {
        if button.just_clicked {
            println!("Click me OK");
            button.just_clicked = false;
        }
    }
}
