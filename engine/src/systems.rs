use macroquad::prelude::*;
use ::rand::{seq::IteratorRandom, thread_rng, Rng};
use crate::context::Context;
use crate::components::*;
use crate::components::{Behavior, Direction,State};
use crate::tiled_map::TileMapComponent;

pub fn movement_system(ctx: &mut Context) {
    for (_, (transform, velocity, speed)) in ctx.world.query::<(&mut Transform, &mut Velocity, &Speed)>().iter() {
        if velocity.0.length() > 0.0 {
            velocity.0 = velocity.0.normalize();
        }
        transform.position += velocity.0 * speed.0 * ctx.dt;
    }
}

pub fn player_update(ctx: &mut Context) {
    for (_, (velocity, state, direction, animation_comp)) in ctx.world.query::<(&mut Velocity, &mut StateComponent, &mut DirectionComponent, &mut AnimationComponent)>().with::<&PlayerTag>().iter() {
        velocity.0 = Vec2::ZERO; // Réinitialiser à chaque frame
        let mut moving = false;
        
        // 1. Définir la vélocité et la direction
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
        
        // 2. Mettre à jour l'état
        state.0 = if moving { State::Walk } else { State::Idle };
        animation_comp.0 = format!("player_base_{}_{}", state.0.to_str(), direction.0.to_str());
    }
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

pub fn update_animations(ctx: &mut Context) {
    for (_, animation_comp) in ctx.world.query::<&AnimationComponent>().iter() {
        if let Some(animation) = ctx.asset_server.get_animation_mut(&animation_comp.0) {
            animation.update(ctx.dt);
        }
    }
}

pub fn tiled_map_render_system(ctx: &mut Context) {
    for (_, tileset_comp) in ctx.world.query::<&TileMapComponent>().iter() {
        if let Some(rendered_map) = ctx.asset_server.get_renderer_map(&tileset_comp.name) {
            draw_texture_ex(
                &rendered_map.texture.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rendered_map.width, rendered_map.height)),
                    ..Default::default()
                }
            );
        }
    }
}

pub fn entities_render_system(ctx: &mut Context) {
    for (_, (animation_comp, transform)) in ctx.world.query::<(&AnimationComponent, &Transform)>().iter() {
        if let Some(animation) = ctx.asset_server.get_animation_mut(&animation_comp.0) {
            animation.draw(transform.position.x, transform.position.y);
        }
    }
}
