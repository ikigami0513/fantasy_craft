use std::{collections::HashMap, sync::Arc};
use macroquad::audio::Sound;
use macroquad::prelude::*;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use crate::graphics::animations::{Animation, AnimationKeyFrame};
use crate::graphics::sprites::{Spritesheet};
use crate::graphics::tiled_map::tiled_map::{Tileset, TileMap, RenderedTileMap};
use crate::graphics::tiled_map::serializers::{LayerData, TiledMapData};

#[derive(Deserialize)]
struct MapData {
    id: String,
    path: String
}

#[derive(Deserialize)]
struct FontData {
    id: String,
    path: String
}

#[derive(Deserialize)]
struct FrameSequenceData {
    row: u32,
    count: u32,
}

#[derive(Deserialize)]
struct SpritesheetData {
    id: String,
    path: String,
    columns: u32,
    rows: u32,
}

#[derive(Deserialize)]
struct AnimationData {
    id: String,
    spritesheet_id: String,
    frame_sequence: FrameSequenceData,
    #[serde(default)] 
    flip: Option<bool>, 
}

#[derive(Deserialize)]
struct SoundData {
    id: String,
    path: String
}

#[derive(Deserialize, Default)]
struct AssetFileData {
    #[serde(default)]
    maps: Vec<MapData>,

    #[serde(default)]
    fonts: Vec<FontData>,

    #[serde(default)]
    spritesheets: Vec<SpritesheetData>,

    #[serde(default)]
    animations: Vec<AnimationData>,

    #[serde(default)]
    sounds: Vec<SoundData>
}

// --- AssetServer ---
pub struct AssetServer {
    animations: HashMap<String, Animation>,
    spritesheets: HashMap<String, Arc<Spritesheet>>,
    maps: HashMap<String, TileMap>,
    rendered_maps: HashMap<String, RenderedTileMap>,
    rendered_layers: HashMap<String, HashMap<String, RenderTarget>>,
    fonts: HashMap<String, Font>,
    sounds: HashMap<String, Sound>
}

#[allow(dead_code)]
impl AssetServer {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            spritesheets: HashMap::new(),
            maps: HashMap::new(),
            rendered_maps: HashMap::new(),
            rendered_layers: HashMap::new(),
            fonts: HashMap::new(),
            sounds: HashMap::new()
        }
    }

    pub fn add_spritesheet(&mut self, name: String, spritesheet: Spritesheet) {
        let arc_spritesheet = Arc::new(spritesheet);
        self.spritesheets.insert(name, arc_spritesheet);
    }

    pub fn add_animation(&mut self, name: String, animation: Animation) {
        self.animations.insert(name, animation);
    }

    pub fn get_spritesheet(&self, name: &str) -> Option<&Arc<Spritesheet>> {
        self.spritesheets.get(name)
    }

    pub fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }

    pub fn get_animation_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.animations.get_mut(name)
    }

    pub fn get_map(&self, name: &str) -> Option<&TileMap> {
        self.maps.get(name)
    }

    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    pub fn get_renderer_map(&self, id: &str) -> Option<&RenderedTileMap> {
        self.rendered_maps.get(id)
    }

    pub fn get_renderer_layer(&self, map_id: &str, layer_name: &str) -> Option<&RenderTarget> {
        self.rendered_layers.get(map_id).and_then(|layers| layers.get(layer_name))
    }

    pub async fn load_sound(&mut self, name: &str, path: &str) {
        match macroquad::audio::load_sound(path).await {
            Ok(sound) => {
                self.sounds.insert(name.to_string(), sound);
            }
            Err(e) => eprintln!("Failed to load sound {}: {:?}", path, e)
        }
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    pub fn merge(&mut self, other: AssetServer) {
        self.animations.extend(other.animations);
        self.spritesheets.extend(other.spritesheets);
        self.fonts.extend(other.fonts);
        self.sounds.extend(other.sounds);
        self.maps.extend(other.maps);
    }

    pub async fn load_tiled_map(&mut self, id: String, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = std::fs::read_to_string(path)?;
        let map_data: TiledMapData = serde_json::from_str(&json_content)?;

        let map_path = Path::new(path);
        let map_dir = map_path.parent().unwrap_or(Path::new(""));

        let mut tilesets = Vec::new();
        let mut tile_layers = HashMap::new();

        for ts_data in &map_data.tilesets {
            let relative_image_path = Path::new(&ts_data.image); 
            let absolute_image_path: PathBuf = map_dir.join(relative_image_path);
            let tileset_path = absolute_image_path.to_str().unwrap().to_string();

            if !self.spritesheets.contains_key(&tileset_path) {
                let texture = load_texture(&tileset_path).await?;
                texture.set_filter(FilterMode::Nearest);

                let tile_w = ts_data.tilewidth as f32;
                let tile_h = ts_data.tileheight as f32;
                let spritesheet = Spritesheet::new(texture, tile_w, tile_h);
                self.add_spritesheet(tileset_path.clone(), spritesheet);
            }

            let spritesheet_arc = self.spritesheets.get(&tileset_path).unwrap().clone();

            tilesets.push(Tileset {
                first_gid: ts_data.firstgid,
                spritesheet: spritesheet_arc,
                columns: ts_data.columns,
                tile_width: map_data.tilewidth as f32,
                tile_height: map_data.tileheight as f32
            });
        }

        tilesets.sort_by_key(|ts| ts.first_gid);

        for layer in map_data.layers {
            if let LayerData::TileLayer { name, data, visible, .. } = layer {
                if visible {
                    tile_layers.insert(name, data);
                }
            }
        }

        let tile_map = TileMap {
            width: map_data.width,
            height: map_data.height,
            tile_width: map_data.tilewidth,
            tile_height: map_data.tileheight,
            tile_layers,
            tilesets,
        };

        self.maps.insert(id, tile_map);
        Ok(())
    }

    pub async fn prepare_loaded_tiledmap(&mut self) {
        for (id, map) in self.maps.iter() {
            let renderer_map = map.to_render_tilemap().await;
            self.rendered_maps.insert(id.clone(), renderer_map);

            let layers = map.render_all_layers().await;
            self.rendered_layers.insert(id.clone(), layers);
        }
    }

    pub async fn finalize_textures(&self) {
        // Draw a single invisible pixel from each texture to ensure GPU upload
        clear_background(BLACK);

        for spritesheet in self.spritesheets.values() {
            draw_texture_ex(
                &spritesheet.texture.clone(),
                -1000.0, -1000.0, // off-screen
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(1.0, 1.0)),
                    ..Default::default()
                },
            );
        }

        // Force GPU sync — let Macroquad flush commands
        next_frame().await;

        // Second frame ensures all textures are uploaded and cached
        next_frame().await;
    }

    // --- Logique de chargement ---
    pub async fn load_assets_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = std::fs::read_to_string(path)?;
        let asset_data: AssetFileData = serde_json::from_str(&json_content)?;

        for map_data in asset_data.maps {
            self.load_tiled_map(map_data.id, &map_data.path).await.unwrap();
        }

        // 1. Chargement des Spritesheets
        for ss_data in asset_data.spritesheets {
            let texture = load_texture(&ss_data.path).await?;

            texture.set_filter(FilterMode::Nearest);
            
            // Note: La taille des sprites est déduite de la taille totale de l'image
            let sprite_width = texture.width() / ss_data.columns as f32;
            let sprite_height = texture.height() / ss_data.rows as f32;

            let spritesheet = Spritesheet::new(texture, sprite_width, sprite_height);
            self.add_spritesheet(ss_data.id, spritesheet);
        }

        // 2. Création des Animations
        for anim_data in asset_data.animations {
            let spritesheet_arc = self.spritesheets.get(&anim_data.spritesheet_id)
                .ok_or_else(|| format!("Spritesheet '{}' not found for animation '{}'", 
                                       anim_data.spritesheet_id, anim_data.id))?
                .clone();

            let frames: Vec<AnimationKeyFrame> = (0..anim_data.frame_sequence.count)
                .map(|col| AnimationKeyFrame::new(col, anim_data.frame_sequence.row))
                .collect();

            let speed = 6.0; 
            let flip_x = anim_data.flip.unwrap_or(false);

            let animation = Animation::new(
                spritesheet_arc, 
                frames, 
                speed,
                flip_x
            );
            
            self.add_animation(anim_data.id, animation);
        }

        // 3. Chargement des fonts
        for font_data in asset_data.fonts {
            let font = load_ttf_font(&font_data.path).await.unwrap();
            self.fonts.insert(font_data.id.clone(), font);
        }

        // 4. Chargement des sons
        for sound_data in asset_data.sounds {
            self.load_sound(&sound_data.id, &sound_data.path).await;
        }

        Ok(())
    }
}
