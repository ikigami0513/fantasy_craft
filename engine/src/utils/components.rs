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

#[derive(Debug)]
pub struct DirectionComponent(pub Direction);

#[derive(Debug)]
pub struct StateComponent(pub State);

#[derive(Debug)]
pub struct Visible(pub bool);

#[derive(Debug)]
pub struct LocalVisible(pub bool);
