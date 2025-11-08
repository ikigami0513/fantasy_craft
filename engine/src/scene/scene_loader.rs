use hecs::Entity;
use serde_json::Value;
use std::collections::HashMap;
use crate::core::context::Context;
use crate::prelude::Parent;
use crate::scene::scene_format::SceneFile;
use std::error::Error;

pub trait ComponentLoader: Send + Sync + 'static {
    fn load(&self, ctx: &mut Context, entity: Entity, data: &Value);
}

pub struct SceneLoader {
    component_loaders: HashMap<String, Box<dyn ComponentLoader>>
}

impl SceneLoader {
    pub fn new() -> Self {
        Self {
            component_loaders: HashMap::new()
        }
    }

    pub fn register<S: Into<String>>(&mut self, name: S, loader: Box<dyn ComponentLoader>) -> &mut Self {
        self.component_loaders.insert(name.into(), loader);
        self
    }

    pub async fn load_scene_from_file(&self, path: &str, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
        let json_content = std::fs::read_to_string(path)?;
        let scene_data: SceneFile = serde_json::from_str(&json_content)?;
        self.load_scene(scene_data, ctx);
        Ok(())
    }

    pub fn load_scene(&self, scene: SceneFile, ctx: &mut Context) {
        let mut entity_map: HashMap<String, Entity> = HashMap::new();
        let mut parent_queue: Vec<(Entity, String)> = Vec::new();

        for entity_data in scene.entities {
            let entity = ctx.world.spawn(());
            entity_map.insert(entity_data.id.clone(), entity);

            for (component_name, component_data) in entity_data.components {
                // --- Gestion spéciale des relations ---
                // Le `Parent` est un cas spécial car il dépend d'autres entités
                // qui ne sont peut-être pas encore créées.
                if component_name == "Parent" {
                    if let Some(target_id) = component_data.as_str() {
                        parent_queue.push((entity, target_id.to_string()));
                    }
                    else {
                        println!(
                            "Warning: La cible 'Parent' pour l'entité {} n'est pas une chaîne de caractères valide.", 
                            entity_data.id
                        );
                    }
                    continue;
                }

                if let Some(loader) = self.component_loaders.get(&component_name) {
                    loader.load(ctx, entity, &component_data);
                }
                else {
                    println!("Warning: Aucun chargeur de composant n'est enregistré pour '{}'", component_name);
                }
            }
        }

        for (entity, parent_id) in parent_queue {
            if let Some(parent_entity) = entity_map.get(&parent_id) {
                ctx.world.insert_one(entity, Parent(*parent_entity)).expect("Failed to add Parent component");
            }
            else {
                println!("Warning: Entité parente non trouvée avec l'ID '{}'", parent_id);
            }
        }
    }
}
