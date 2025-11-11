use crate::{core::plugins::Plugin, physics::systems::physics_system, prelude::{ColliderLoader, GameState, RigidBodyLoader, SpeedLoader, Stage, System, TransformLoader, VelocityLoader, movement_system}};

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
            .add_system(Stage::Update, System::new(
                movement_system,
                vec![GameState::Playing]
            ))
            .add_system(Stage::PostUpdate, System::new(
                physics_system,
                vec![GameState::Playing]
            ));
    }
}
