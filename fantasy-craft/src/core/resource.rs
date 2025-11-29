use std::any::{Any, TypeId};

use parry2d::utils::hashmap::HashMap;

type AnySend = dyn Any + Send;

#[derive(Default)]
pub struct ResourceMap {
    storage: HashMap<TypeId, Box<AnySend>>
}

impl ResourceMap {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Inserts a resource into the map.
    /// If a resource of this type already exists, it is overwritten.
    ///
    /// `T` must be `Any + 'static`
    pub fn insert<T: Any + Send + 'static>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.storage.insert(type_id, Box::new(resource));
    }

    /// Gets an immutable reference to a resource of type `T`.
    /// Returns `None` if the resource does not exist.
    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.storage
            .get(&type_id)
            .and_then(|boxed_value| boxed_value.downcast_ref::<T>())
    }

    /// Gets a mutable reference to a resource of type `T`.
    /// Returns `None` if the resource does not exist.
    pub fn get_mut<T: Any + Send + 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.storage
            .get_mut(&type_id)
            .and_then(|boxed_value| boxed_value.downcast_mut::<T>())
    }

    /// Removes and returns a resource of type `T`.
    /// Returns `None` if the resource does not exist.
    pub fn remove<T: Any + Send + 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.storage
            .remove(&type_id)
            .and_then(|boxed_value| boxed_value.downcast::<T>().ok())
            .map(|unboxed_value| *unboxed_value)
    }
}