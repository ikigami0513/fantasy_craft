use std::any::Any;

use hecs::World;

use crate::{core::{resource::ResourceMap, time::DeltaTime}, prelude::{AssetServer, GameState}};

pub struct Context {
    pub world: World,
    pub asset_server: AssetServer,
    pub game_state: GameState,
    pub resources: ResourceMap
}

impl Context {
    pub fn new(world: World, asset_server: AssetServer) -> Self {
        let resources = ResourceMap::new();

        Self {
            world,
            asset_server,
            game_state: GameState::Playing,
            resources
        }
    }

    /// Inserts a new resource.
    pub fn insert_resource<T: Any + Send + 'static>(&mut self, resource: T) {
        self.resources.insert(resource);
    }

    /// Gets an immutable reference to a resource.
    /// Panics if the resource is not found.
    pub fn resource<T: Any + 'static>(&self) -> &T {
        self.get_resource::<T>()
            .unwrap_or_else(|| panic!("Resource not found: {}", std::any::type_name::<T>()))
    }

    /// Gets a mutable reference to a resource.
    /// Panics if the resource is not found.
    pub fn resource_mut<T: Any + Send + 'static>(&mut self) -> &mut T {
        self.get_resource_mut::<T>()
            .unwrap_or_else(|| panic!("Resource not found: {}", std::any::type_name::<T>()))
    }

    /// Tries to get an immutable reference to a resource.
    /// This is the method you asked for.
    pub fn get_resource<T: Any + 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    /// Tries to get a mutable reference to a resource.
    pub fn get_resource_mut<T: Any + Send + 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn dt(&self) -> f32 {
        self.resource::<DeltaTime>().0
    }
}
