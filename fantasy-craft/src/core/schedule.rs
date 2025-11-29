use std::collections::BTreeMap;

use crate::core::context::Context;

pub type SystemFn = fn(&mut Context);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    StartUp,
    Update,
    PostUpdate,
    Render,
    PostRender,
    GuiRender
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    Loading
}

pub struct System {
    pub func: SystemFn,
    pub active_states: Vec<GameState>
}

impl System {
    pub fn new(func: SystemFn, active_states: Vec<GameState>) -> Self {
        Self {
            func,
            active_states
        }
    }

    pub fn is_active(&self, current_state: GameState) -> bool {
        self.active_states.contains(&current_state)
    }
}

pub struct Schedule {
    systems: BTreeMap<Stage, Vec<System>>
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            systems: BTreeMap::new()
        }
    }

    pub fn add_system(&mut self, stage: Stage, system: System) {
        self.systems.entry(stage).or_default().push(system);
    }

    pub fn run_stage(&self, stage: Stage, ctx: &mut Context) {
        if let Some(systems) = self.systems.get(&stage) {
            for system in systems {
                if system.is_active(ctx.game_state) {
                    (system.func)(ctx);
                }
            }
        }
    }
}
