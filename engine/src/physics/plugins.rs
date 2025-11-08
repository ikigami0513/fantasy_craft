use crate::{core::plugins::Plugin, physics::systems::physics_system, prelude::{ColliderLoader, RigidBodyLoader, SpeedLoader, Stage, TransformLoader, VelocityLoader, movement_system}};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.scene_loader
            .register("Transform", Box::new(TransformLoader))
            .register("RigidBody", Box::new(RigidBodyLoader))
            .register("Collider", Box::new(ColliderLoader))
            .register("Velocity", Box::new(VelocityLoader))
            .register("Speed", Box::new(SpeedLoader));

        app
            .add_system(Stage::Update, movement_system)
            .add_system(Stage::PostUpdate, physics_system);
    }
}
