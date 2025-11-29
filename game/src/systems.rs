use std::process::exit;

use hecs::Entity;
use macroquad::prelude::*;
// removed: use macroquad::rand::*; // 'prelude' already includes rand, but explicit use is fine too if you prefer.
// removed: use ::rand::{seq::IteratorRandom, thread_rng, Rng}; // We don't need the external rand crate anymore.

use fantasy_craft::{audio::event::PlaySoundEvent, core::event::EventBus, gui::{event::UiClickEvent}, input::{focus::InputFocus, manager::InputManager}, prelude::*};
use crate::components::{AnimationPrefix, Behavior, BehaviorComponent, FpsDisplay, MainMenu, NpcTag, PlayerTag};
use fantasy_craft::gui::text_display::TextDisplay;

/// System handling NPC logic using Macroquad's RNG
pub fn npc_behavior_system(ctx: &mut Context) {
    // We iterate over all entities with the required components
    for (_, (transform, npc, behavior, state, direction, speed, animation_comp)) in ctx.world.query::<(&mut Transform, &mut NpcTag, &BehaviorComponent, &mut StateComponent, &mut DirectionComponent, &Speed, &mut AnimationComponent)>().iter() {
        match behavior.0 {
            Behavior::Stand => {
                state.0 = State::Idle;
            },
            Behavior::Wander => {
                npc.wander_time += ctx.dt();
                
                // Logic when the wander timer exceeds the target duration
                if npc.wander_time >= npc.wander_target_duration {
                    npc.wander_time = 0.0;

                    if state.0 == State::Idle {
                        state.0 = State::Walk;
                        
                        // Define available directions
                        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
                        
                        // MACROQUAD CHANGE: 
                        // Instead of using iterator.choose(), we generate a random index.
                        // gen_range(min, max) with integers excludes the max, so 0..len is correct.
                        let random_index = rand::gen_range(0, directions.len());
                        direction.0 = directions[random_index];

                        // MACROQUAD CHANGE:
                        // Replaced rng.gen_range(1.0..=3.0) with rand::gen_range(min, max)
                        npc.wander_target_duration = rand::gen_range(1.0, 3.0);
                    }
                    else {
                        state.0 = State::Idle;
                        // MACROQUAD CHANGE:
                        // Replaced rng.gen_range(2.0..=5.0) with rand::gen_range(min, max)
                        npc.wander_target_duration = rand::gen_range(2.0, 5.0);
                    }
                }

                // Movement logic based on direction
                if state.0 == State::Walk {
                    let direction_vec = match direction.0 {
                        Direction::Up => vec2(0.0, -1.0),
                        Direction::Down => vec2(0.0, 1.0),
                        Direction::Left => vec2(-1.0, 0.0),
                        Direction::Right => vec2(1.0, 0.0)
                    };
                    transform.position += direction_vec * speed.0 * ctx.dt();
                }
            }
        }

        // Update animation string key
        animation_comp.0 = format!("{}_{}_{}", npc.name, state.0.to_str(), direction.0.to_str());
    }
}

pub fn player_update(ctx: &mut Context) {
    // Check if UI has captured input to prevent character movement during UI interaction
    let input_is_captured = if let Some(input_focus) = ctx.get_resource::<InputFocus>() {
        input_focus.is_captured_by_ui
    } else {
        false
    };

    let player_entities: Vec<Entity> = ctx.world.query::<&PlayerTag>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    let (_world, resources) = (&mut ctx.world, &ctx.resources);

    // Access InputManager from resources
    let input = resources.get::<InputManager>().expect("InputManager missing");

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
            if input.is_action_down("MoveRight") { 
                velocity.0.x = 1.0; 
                direction.0 = Direction::Right;
                moving = true;
            }
            if input.is_action_down("MoveLeft") { 
                velocity.0.x = -1.0; 
                direction.0 = Direction::Left;
                moving = true;
            }
            if input.is_action_down("MoveUp") { 
                velocity.0.y = -1.0; 
                // Prioritize vertical movement direction only if horizontal keys aren't pressed (simple 4-way logic)
                if !is_key_down(KeyCode::Right) && !is_key_down(KeyCode::Left) {
                    direction.0 = Direction::Up;
                }
                moving = true;
            }
            if input.is_action_down("MoveDown") { 
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

        // Update children animations (e.g., for composite sprites like body + armor)
        for child in children {
            if let Ok((animation_prefix, animation_comp)) = ctx.world.query_one_mut::<(&AnimationPrefix, &mut AnimationComponent)>(child) {
                animation_comp.0 = format!("{}_{}_{}", animation_prefix.0, current_state.to_str(), current_direction.to_str());
            }
        }
    }
}

pub fn fps_display_update(ctx: &mut Context) {
    for (_, (fps_display, text_display)) in ctx.world.query::<(&mut FpsDisplay, &mut TextDisplay)>().iter() {
        fps_display.fps_timer += ctx.dt();

        if fps_display.fps_timer >= 1.0 {
            fps_display.displayed_fps = get_fps();
            fps_display.fps_timer = 0.0;
            text_display.text = format!("FPS: {}", fps_display.displayed_fps);
        }
    }
}

pub fn check_player_npc_collision(ctx: &mut Context) {
    // 1. Access the EventBus resource
    let event_bus = ctx.resource::<EventBus>();

    // 2. Read specific events using the generic read method.
    for event in event_bus.read::<CollisionEvent>() {
        let e_a = event.entity_a;
        let e_b = event.entity_b;

        let a_is_player = ctx.world.get::<&PlayerTag>(e_a).is_ok();
        let b_is_player = ctx.world.get::<&PlayerTag>(e_b).is_ok();

        let a_is_npc = ctx.world.get::<&NpcTag>(e_a).is_ok();
        let b_is_npc = ctx.world.get::<&NpcTag>(e_b).is_ok();

        // Respond to specific collision pairs
        if a_is_player && b_is_npc {
            println!("💥 Collision detected! Player ({:?}) hit NPC ({:?})", e_a, e_b);
        }
        else if b_is_player && a_is_npc {
            println!("💥 Collision detected! NPC ({:?}) hit Player ({:?})", e_a, e_b);
        }
    }
}

pub fn menu_buttons_system(ctx: &mut Context) {
    let mut should_quit = false;
    let mut sound_to_play: Option<String> = None;

    // --- READING PHASE ---
    {
        let event_bus = ctx.resource::<EventBus>();

        for event in event_bus.read::<UiClickEvent>() {
            match event.action_id.as_str() {
                "quit_game" => {
                    should_quit = true;
                },
                "test_button" => {
                    sound_to_play = Some("button_click".to_string());
                },
                _ => println!("Unknown action : {}", event.action_id)
            }
        }
    } 

    // --- WRITING PHASE ---

    if let Some(sound_name) = sound_to_play {
        ctx.resource_mut::<EventBus>().send(PlaySoundEvent {
            sound_name
        });
    }

    if should_quit {
        println!("Bye Fantasy Craft");
        exit(0);
    }
}

pub fn toggle_main_menu_system(ctx: &mut Context) {
    let input = ctx.resource::<InputManager>();

    if input.is_action_just_pressed("Menu") {
        for (_, (_main_menu, visible)) in ctx.world.query::<(&MainMenu, &mut Visible)>().iter() {
            visible.0 = !visible.0;

            ctx.game_state = if visible.0 {
                GameState::Menu
            } else {
                GameState::Playing
            };
        }
    }
}
