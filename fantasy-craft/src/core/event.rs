use std::any::{Any, TypeId};
use std::collections::HashMap;

// --- Internal Trait ---
// Allows us to treat generic EventQueues as dynamic objects
// so we can store them in a HashMap.
trait EventQueueTrait: Any + Send + Sync {
    fn clear(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// --- Generic Queue ---
// Holds the actual events of type E.
struct EventQueue<E> {
    events: Vec<E>
}

impl<E: 'static + Send + Sync> EventQueueTrait for EventQueue<E> {
    fn clear(&mut self) {
        self.events.clear();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<E> Default for EventQueue<E> {
    fn default() -> Self {
        Self {
            events: Vec::new()
        }
    }
}

// --- The Event Bus Resource ---
#[derive(Default)]
pub struct EventBus {
    queues: HashMap<TypeId, Box<dyn EventQueueTrait>>
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an event into the queue for the current frame.
    pub fn send<E: 'static + Send + Sync>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();

        // Get or create the queue for this event type
        let queue = self.queues
            .entry(type_id)
            .or_insert_with(|| Box::new(EventQueue::<E>::default()));

        // Downcast to the concrete type and push
        if let Some(q) = queue.as_any_mut().downcast_mut::<EventQueue<E>>() {
            q.events.push(event);
        }
    }

    /// Returns an iterator over events of type E sent this frame.
    pub fn read<E: 'static + Send + Sync>(&self) -> impl Iterator<Item = &E> {
        let type_id = TypeId::of::<E>();

        if let Some(queue) = self.queues.get(&type_id) {
            if let Some(q) = queue.as_any().downcast_ref::<EventQueue<E>>() {
                // We return an iterator over the slice
                return Some(q.events.iter()).into_iter().flatten();
            }
        }

        // If queue doesn't exist, return an empty iterator
        None.into_iter().flatten()
    }

    pub fn clear(&mut self) {
        for queue in self.queues.values_mut() {
            queue.clear();
        }
    }
}
