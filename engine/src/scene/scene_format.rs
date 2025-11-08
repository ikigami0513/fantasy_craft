use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct EntityData {
    pub id: String,

    #[serde(default)]
    pub components: HashMap<String, Value>
}

#[derive(Deserialize, Debug)]
pub struct SceneFile {
    pub entities: Vec<EntityData>
}
