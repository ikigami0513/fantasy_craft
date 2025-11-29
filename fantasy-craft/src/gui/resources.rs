use std::collections::HashMap;
use hecs::Entity;
use macroquad::prelude::*;

/// Wrapper for the UI rectangle cache.
#[derive(Debug, Default)]
pub struct UiResolvedRects(pub HashMap<Entity, (Vec2, Vec2)>);

#[derive(Debug, Clone, Copy)]
pub struct PreviousMousePosition(pub Vec2);
