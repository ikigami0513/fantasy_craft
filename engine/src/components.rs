use macroquad::prelude::*;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Behavior {
    Stand,
    Wander
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: Vec2,
    pub scale: Vec2
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            rotation: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0)
        }
    }
}

#[derive(Debug)]
pub struct Velocity(pub Vec2);

#[derive(Debug)]
pub struct DirectionComponent(pub Direction);

#[derive(Debug)]
pub struct StateComponent(pub State);

#[derive(Debug)]
pub struct AnimationComponent(pub String);

#[derive(Debug)]
pub struct PlayerTag;

#[derive(Debug)]
pub struct NpcTag {
    pub name: String,
    pub wander_time: f32,
    pub wander_target_duration: f32
}

#[derive(Debug)]
pub struct BehaviorComponent(pub Behavior);

#[derive(Debug)]
pub struct Speed(pub f32);