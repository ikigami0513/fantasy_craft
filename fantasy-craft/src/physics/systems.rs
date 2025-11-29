use hecs::Entity;
use macroquad::prelude::*;
use parry2d::query;

use crate::core::event::EventBus;
use crate::physics::components::Transform;
use crate::core::context::Context;
use crate::physics::components::{BodyType, Collider, RigidBody, Velocity, Speed};
use crate::physics::helpers::make_isometry;
use crate::prelude::CollisionEvent;

pub fn movement_system(ctx: &mut Context) {
    // Obtenez dt AVANT la boucle
    let dt = ctx.dt(); 

    // query_mut() est plus idiomatique que query::<...>().iter() pour des &mut
    for (_, (transform, velocity, speed)) in ctx.world.query_mut::<(&mut Transform, &mut Velocity, &Speed)>() {
        if velocity.0.length() > 0.0 {
            velocity.0 = velocity.0.normalize();
        }
        // Utilisez la variable dt locale
        transform.position += velocity.0 * speed.0 * dt;
    }
}

pub fn physics_system(ctx: &mut Context) {
    
    // --- Phase 1: Preparation & Disjoint Borrows ---
    
    let dt = ctx.dt(); 
    
    // Split context to access world and resources separately
    let (world, resources) = (&mut ctx.world, &mut ctx.resources);

    // Get the EventBus
    let event_bus = resources.get_mut::<EventBus>()
        .expect("EventBus resource is missing");

    // --- Phase 2: Collect (Read Only from World) ---
    
    let mut entities: Vec<(Entity, Vec2, RigidBody, Velocity, Collider)> = Vec::new();

    // Query and Clone (this releases the world borrow immediately after the loop)
    for (entity, (transform, rigidbody, velocity, collider)) in 
        world.query::<(&Transform, &RigidBody, &Velocity, &Collider)>().iter() 
    {
        entities.push((
            entity,
            transform.position,
            rigidbody.clone(), 
            velocity.clone(),  
            collider.clone()   
        ));
    }
    
    // --- Phase 3: Simulation ---
    
    // Step 1: Integration
    for (_, position, rb, velocity, _) in entities.iter_mut() {
        if let BodyType::Dynamic = rb.body_type {
            *position += velocity.0 * dt;
        }
    }

    // Step 2: Collision Resolution
    let mut i = 0;
    while i < entities.len() {
        let (left, right) = entities.split_at_mut(i + 1);
        let entity_a = &mut left[i];
        
        for entity_b in right.iter_mut() {
            
            let iso_a = make_isometry(entity_a.1);
            let iso_b = make_isometry(entity_b.1);

            if let Ok(Some(contact)) = query::contact(&iso_a, &*entity_a.4.shape, &iso_b, &*entity_b.4.shape, 0.0) {
                let normal_vector = contact.normal1.into_inner();
                let half_correction = normal_vector * contact.dist * 0.5;

                // Physics Response (Pushing objects apart)
                if matches!(entity_a.2.body_type, BodyType::Dynamic) && matches!(entity_b.2.body_type, BodyType::Static) {
                    let correction = normal_vector * contact.dist;
                    entity_a.1 += vec2(correction.x, correction.y); 
                } 
                else if matches!(entity_b.2.body_type, BodyType::Dynamic) && matches!(entity_a.2.body_type, BodyType::Static) {
                    let correction = -normal_vector * contact.dist;
                    entity_b.1 += vec2(correction.x, correction.y);
                }
                else if matches!(entity_a.2.body_type, BodyType::Dynamic) && matches!(entity_b.2.body_type, BodyType::Dynamic) {
                    entity_a.1 += vec2(half_correction.x, half_correction.y);
                    entity_b.1 -= vec2(half_correction.x, half_correction.y);
                }

                // --- NEW: Send Event via EventBus ---
                // We use the disjoint borrow 'event_bus' here.
                event_bus.send(CollisionEvent {
                    entity_a: entity_a.0,
                    entity_b: entity_b.0
                });
            }
        }
        i += 1;
    }

    // --- Phase 4: Write Back ---
    
    for (entity, new_pos, _, _, _) in entities {
        if let Ok(mut transform) = world.get::<&mut Transform>(entity) {
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
