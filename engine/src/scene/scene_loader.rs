// In your scene_loader.rs

use hecs::Entity;
use serde_json::Value;
use std::collections::HashMap;
use crate::core::context::Context;
use crate::prelude::Parent;
// MODIFIED: Import the new SceneFile and SceneEntry
use crate::scene::scene_format::{SceneFile, SceneEntry}; 
use std::error::Error;
use std::path::Path;
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

    // This is the main public entry point.
    // It sets up the maps and calls the internal recursive loader.
    pub async fn load_scene_from_file(&self, path: &str, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
        
        // These maps are shared across all recursive calls
        // to ensure all entities are registered centrally.
        let mut entity_map: HashMap<String, Entity> = HashMap::new();
        
        // The parent queue is also collected from all files first.
        let mut parent_queue: Vec<(Entity, String)> = Vec::new();

        // Start the recursive loading process
        self.load_scene_internal(path, ctx, &mut entity_map, &mut parent_queue).await?;

        // After ALL files are loaded and entities spawned,
        // process the parent relationships.
        self.process_parent_queue(ctx, &entity_map, parent_queue);

        Ok(())
    }

    // Internal recursive function to load scenes
    #[async_recursion]
    async fn load_scene_internal(
        &self,
        path: &str,
        ctx: &mut Context,
        entity_map: &mut HashMap<String, Entity>,
        parent_queue: &mut Vec<(Entity, String)>,
    ) -> Result<(), Box<dyn Error>> {
        
        // Read and parse the scene file
        let json_content = std::fs::read_to_string(path)?;
        let scene_data: SceneFile = serde_json::from_str(&json_content)?;

        // Get the directory of the current file for resolving relative import paths
        let current_dir = Path::new(path).parent().unwrap_or(Path::new(""));

        // Iterate over the entries (which can be Entities or Imports)
        for entry in scene_data.entities {
            match entry {
                // Case 1: It's a standard entity definition
                SceneEntry::Entity(entity_data) => {
                    let entity = ctx.world.spawn(());
                    
                    // Register the entity in the global map
                    if entity_map.insert(entity_data.id.clone(), entity).is_some() {
                        println!(
                            "Warning: Duplicate entity ID found: '{}'. Overwriting.",
                            entity_data.id
                        );
                    }

                    // Load components as before
                    for (component_name, component_data) in entity_data.components {
                        // Handle Parent component special case
                        if component_name == "Parent" {
                            if let Some(target_id) = component_data.as_str() {
                                // Add to queue; DO NOT process yet
                                parent_queue.push((entity, target_id.to_string()));
                            } else {
                                println!(
                                    "Warning: 'Parent' target for entity {} is not a valid string.",
                                    entity_data.id
                                );
                            }
                            continue; // Skip normal loading
                        }

                        // Load other components
                        if let Some(loader) = self.component_loaders.get(&component_name) {
                            loader.load(ctx, entity, &component_data);
                        } else {
                            println!(
                                "Warning: No component loader registered for '{}'",
                                component_name
                            );
                        }
                    }
                }
                
                // Case 2: It's an import directive
                SceneEntry::Import(import_data) => {
                    // Resolve the relative path
                    let import_path = current_dir.join(&import_data.import);
                    let import_path_str = import_path
                        .to_str()
                        .ok_or("Failed to convert import path to string")?;

                    println!("Importing scene from: {}", import_path_str);

                    // --- RECURSIVE CALL ---
                    // Call self with the new path, passing the *same* maps.
                    self.load_scene_internal(
                        import_path_str,
                        ctx,
                        entity_map,
                        parent_queue,
                    ).await?;
                }
            }
        }

        Ok(())
    }

    // This is called only ONCE after all files are loaded.
    fn process_parent_queue(
        &self,
        ctx: &mut Context,
        entity_map: &HashMap<String, Entity>,
        parent_queue: Vec<(Entity, String)>,
    ) {
        for (entity, parent_id) in parent_queue {
            if let Some(parent_entity) = entity_map.get(&parent_id) {
                ctx.world
                    .insert_one(entity, Parent(*parent_entity))
                    .expect("Failed to add Parent component");
            } else {
                println!(
                    "Warning: Parent entity not found with ID '{}' for child entity {:?}",
                    parent_id, entity
                );
            }
        }
    }
}
