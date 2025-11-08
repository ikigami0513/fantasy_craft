use crate::scene::scene_loader::ComponentLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Down,
    Up,
    Right,
    Left
}

impl Direction {
    pub fn to_str(&self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }

    pub fn from_str(value: &str) -> Direction {
        match value {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => Direction::Down
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Walk
}

impl State {
    pub fn to_str(&self) -> &'static str {
        match self {
            State::Idle => "idle",
            State::Walk => "walk",
        }
    }

    pub fn from_str(value: &str) -> State {
        match value {
            "idle" => State::Idle,
            "walk" => State::Walk,
            _ => Self::Idle
        }
    }
}

#[derive(Debug)]
pub struct DirectionComponent(pub Direction);

pub struct DirectionComponentLoader;

impl ComponentLoader for DirectionComponentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = DirectionComponent(Direction::from_str(&loader_data));

        ctx.world.insert_one(entity, component).expect("Failed to insert DirectionComponent");
    }
}

#[derive(Debug)]
pub struct StateComponent(pub State);

pub struct StateComponentLoader;

impl ComponentLoader for StateComponentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = StateComponent(State::from_str(&loader_data));

        ctx.world.insert_one(entity, component).expect("Failed to insert StateComponent");
    }
}

#[derive(Debug)]
pub struct Visible(pub bool);

pub struct VisibleLoader;

impl ComponentLoader for VisibleLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: bool = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = Visible(loader_data);

        ctx.world.insert_one(entity, component).expect("Failed to insert Visible");
    }
}

#[derive(Debug)]
pub struct LocalVisible(pub bool);

pub struct LocalVisibleLoader;

impl ComponentLoader for LocalVisibleLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: bool = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = LocalVisible(loader_data);

        ctx.world.insert_one(entity, component).expect("Failed to insert LocalVisible");
    }
}
