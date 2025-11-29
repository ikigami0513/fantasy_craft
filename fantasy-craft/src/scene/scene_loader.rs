use macroquad::prelude::*;
use hecs::Entity;
use serde_json::Value;
use std::collections::HashMap;
use crate::core::context::Context;
use crate::prelude::Parent;
use crate::scene::scene_format::{SceneFile, SceneEntry};
// We need to import the Error trait explicitly
use std::error::Error;
use async_recursion::async_recursion;

pub trait ComponentLoader: Send + Sync + 'static {
    fn load(&self, ctx: &mut Context, entity: Entity, data: &Value);
}

pub struct SceneLoader {
    component_loaders: HashMap<String, Box<dyn ComponentLoader>>,
}

impl SceneLoader {
    pub fn new() -> Self {
        Self {
            component_loaders: HashMap::new(),
        }
    }

    pub fn register<S: Into<String>>(&mut self, name: S, loader: Box<dyn ComponentLoader>) -> &mut Self {
        self.component_loaders.insert(name.into(), loader);
        self
    }

    // Public entry point
    pub async fn load_scene_from_file(&self, path: &str, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
        let mut entity_map: HashMap<String, Entity> = HashMap::new();
        let mut parent_queue: Vec<(Entity, String)> = Vec::new();

        info!("SceneLoader: Starting load from root: {}", path);

        self.load_scene_internal(path, ctx, &mut entity_map, &mut parent_queue).await?;

        self.process_parent_queue(ctx, &entity_map, parent_queue);

        info!("SceneLoader: Loading complete.");
        Ok(())
    }

    // Internal recursive function
    // We explicitly cast errors to Box<dyn Error> to satisfy the signature
    #[async_recursion]
    async fn load_scene_internal(
        &self,
        path: &str,
        ctx: &mut Context,
        entity_map: &mut HashMap<String, Entity>,
        parent_queue: &mut Vec<(Entity, String)>,
    ) -> Result<(), Box<dyn Error>> { // <--- The return type we must strictly adhere to
        
        // 1. Load string using Macroquad's HTTP/FS abstraction
        let json_content = load_string(path)
            .await
            .map_err(|e| {
                error!("SceneLoader: Failed to load file '{}': {}", path, e);
                // Explicitly box the macroquad error into the trait object
                Box::new(e) as Box<dyn Error>
            })?;

        // 2. Parse JSON
        let scene_data: SceneFile = serde_json::from_str(&json_content)
            .map_err(|e| {
                error!("SceneLoader: Failed to parse JSON in '{}': {}", path, e);
                // Explicitly box the serde error into the trait object
                Box::new(e) as Box<dyn Error>
            })?;

        // 3. Resolve current directory URL-safely
        let current_dir = if let Some(last_slash_idx) = path.rfind('/') {
            &path[0..=last_slash_idx]
        } else {
            ""
        };

        for entry in scene_data.entities {
            match entry {
                SceneEntry::Entity(entity_data) => {
                    let entity = ctx.world.spawn(());
                    
                    if entity_map.insert(entity_data.id.clone(), entity).is_some() {
                        warn!("Warning: Duplicate entity ID found: '{}'. Overwriting.", entity_data.id);
                    }

                    for (component_name, component_data) in entity_data.components {
                        if component_name == "Parent" {
                            if let Some(target_id) = component_data.as_str() {
                                parent_queue.push((entity, target_id.to_string()));
                            } else {
                                warn!("Warning: 'Parent' target for entity {} is invalid.", entity_data.id);
                            }
                            continue;
                        }

                        if let Some(loader) = self.component_loaders.get(&component_name) {
                            loader.load(ctx, entity, &component_data);
                        } else {
                            warn!("Warning: No component loader registered for '{}'", component_name);
                        }
                    }
                }
                
                SceneEntry::Import(import_data) => {
                    let import_path_str = format!("{}{}", current_dir, import_data.import);

                    info!("Importing sub-scene from: {}", import_path_str);

                    // 4. Recursive call
                    // Since the recursive function already returns Result<(), Box<dyn Error>>,
                    // the ? operator works fine here.
                    self.load_scene_internal(
                        &import_path_str,
                        ctx,
                        entity_map,
                        parent_queue,
                    ).await?;
                }
            }
        }

        Ok(())
    }

    fn process_parent_queue(
        &self,
        ctx: &mut Context,
        entity_map: &HashMap<String, Entity>,
        parent_queue: Vec<(Entity, String)>,
    ) {
        for (entity, parent_id) in parent_queue {
            if let Some(parent_entity) = entity_map.get(&parent_id) {
                if ctx.world.contains(entity) {
                    ctx.world
                        .insert_one(entity, Parent(*parent_entity))
                        .expect("Failed to add Parent component");
                }
            } else {
                warn!("Warning: Parent entity not found with ID '{}' for child entity {:?}", parent_id, entity);
            }
        }
    }
}
