use std::collections::BTreeMap;

use crate::context::Context;

pub type System = fn(&mut Context);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    StartUp,
    Update,
    PostUpdate,
    Render
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
                system(ctx);
            }
        }
    }
}
