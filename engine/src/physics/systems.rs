use hecs::Entity;
use macroquad::prelude::*;
use parry2d::query;

use crate::physics::components::Transform;
use crate::core::context::Context;
use crate::physics::components::{BodyType, Collider, RigidBody, Velocity, Speed};
use crate::physics::helpers::make_isometry;
use crate::prelude::CollisionEvent;

pub fn movement_system(ctx: &mut Context) {
    for (_, (transform, velocity, speed)) in ctx.world.query::<(&mut Transform, &mut Velocity, &Speed)>().iter() {
        if velocity.0.length() > 0.0 {
            velocity.0 = velocity.0.normalize();
        }
        transform.position += velocity.0 * speed.0 * ctx.dt;
    }
}

pub fn physics_system(ctx: &mut Context) {
    // 1. Storage for integration/collision (needs references to RigiBody/Collider)
    let mut entities: Vec<(Entity, Vec2, &mut RigidBody, &Velocity, &Collider)> = Vec::new();
    
    // 2. Storage for final position updates (owned data)
    let mut position_updates: Vec<(hecs::Entity, Vec2)> = Vec::new();

    // The mutable query starts here and lasts until the end of the function, 
    // but we can break it down logically.
    let mut query_borrow = ctx.world.query::<(&mut Transform, &mut RigidBody, &Velocity, &Collider)>();

    // Phase 1: Read/Collect
    for (entity, (transform, rigidbody, velocity, collider)) in query_borrow.iter() {
        entities.push((entity, transform.position, rigidbody, velocity, collider));
    }
    // 'entities' now holds the necessary references. 'query_borrow' is still active.

    // Step 1: integrate motion (using the copied position in 'entities')
    for (_, position, rb, velocity, _) in entities.iter_mut() {
        if let BodyType::Dynamic = rb.body_type {
            *position += velocity.0 * ctx.dt;
        }
    }

    // Step 2: collision resolution (using the copied position in 'entities')
    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let (_, pos_a, rb_a, _, col_a) = &entities[i];
            let (_, pos_b, rb_b, _, col_b) = &entities[j];

            let iso_a = make_isometry(*pos_a);
            let iso_b = make_isometry(*pos_b);

            if let Ok(Some(contact)) = query::contact(&iso_a, &*col_a.shape, &iso_b, &*col_b.shape, 0.0) {
                let normal_vector = contact.normal1.into_inner();
                
                let half_correction = normal_vector * contact.dist * 0.5;

                // Dynamic vs Static
                if matches!(rb_a.body_type, BodyType::Dynamic) && matches!(rb_b.body_type, BodyType::Static) {
                    // Push A of the total distance
                    let correction = normal_vector * contact.dist;
                    entities[i].1 += vec2(correction.x, correction.y); 
                } 
                // Static vs Dynamic
                else if matches!(rb_b.body_type, BodyType::Dynamic) && matches!(rb_a.body_type, BodyType::Static) {
                    // Push B the full distance (in the opposite direction)
                    let correction = -normal_vector * contact.dist;
                    entities[j].1 += vec2(correction.x, correction.y);
                }
                // 🆕 Dynamic vs Dynamic
                else if matches!(rb_a.body_type, BodyType::Dynamic) && matches!(rb_b.body_type, BodyType::Dynamic) {
                    // Push A by 50%
                    entities[i].1 += vec2(half_correction.x, half_correction.y);
                    // Push B by 50% (in the opposite direction)
                    entities[j].1 -= vec2(half_correction.x, half_correction.y);
                }

                // TODO: Dynamic vs Kinematic and Static vs Static

                ctx.collision_events.push(CollisionEvent {
                    entity_a: entities[i].0,
                    entity_b: entities[j].0
                });
            }
        }
    }

    // Step 3: Collect updates (The references are no longer needed)
    // We now fill the list of owned updates
    for (entity, new_pos, _, _, _) in entities {
        position_updates.push((entity, new_pos));
    }
    
    // The scope of 'query_borrow' effectively ends here since it's no longer used, 
    // even though it was declared earlier.
    drop(query_borrow); 

    // Phase 2: Write Back (Safe World Access)
    // We can now safely iterate over the owned data in 'position_updates' 
    // and access the world entity-by-entity.
    for (entity, new_pos) in position_updates {
        // This is safe because no other query/borrow is currently active.
        if let Ok(mut transform) = ctx.world.get::<&mut Transform>(entity) {
            transform.position = new_pos;
        }
    }
}


pub fn collider_debug_render_system(ctx: &mut Context) {
    for (_, (transform, collider)) in ctx.world.query::<(&Transform, &Collider)>().iter() {
        let position = transform.position;
        let half_extents = collider.half_extents;

        let width = half_extents.x * 2.0;
        let height = half_extents.y * 2.0;

        let x = position.x - half_extents.x;
        let y = position.y - half_extents.y;

        draw_rectangle_lines(x, y, width, height, 2.0, RED);
    }
}
