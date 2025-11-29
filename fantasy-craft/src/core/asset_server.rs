use std::{collections::HashMap, sync::Arc};
use macroquad::audio::Sound;
use macroquad::prelude::*;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use crate::graphics::animations::{Animation, AnimationKeyFrame};
use crate::graphics::sprites::{Spritesheet};
use crate::graphics::tiled_map::tiled_map::{Tileset, TileMap, RenderedTileMap};
use crate::graphics::tiled_map::serializers::{LayerData, TiledMapData};
// Assure-toi d'importer WebContext
use crate::core::web_context::WebContext; 

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

    // --- Helper pour concaténer l'URL de base et le chemin relatif ---
    fn resolve_path(base: &str, path: &str) -> String {
        // Si le chemin est déjà absolu (http...), on ne touche à rien
        if path.starts_with("http") {
            return path.to_string();
        }
        format!("{}{}", base, path)
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
            Err(e) => error!("Failed to load sound {}: {:?}", path, e)
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
        // CORRECTION MAJEURE : Utilisation de load_string (HTTP) au lieu de std::fs (Disque)
        let json_content = match load_string(path).await {
            Ok(s) => s,
            Err(e) => {
                error!("Impossible de charger la map {}. Erreur: {}", path, e);
                return Err(Box::new(e));
            }
        };

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
                // Ici, load_texture utilisera l'URL complète car tileset_path est dérivé de path (qui est déjà une URL)
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
        #[cfg(not(target_arch = "wasm32"))]
        {
            info!("Natif détecté : Préparation des RenderTargets pour les maps...");
            for (id, map) in self.maps.iter() {
                // On garde ton code actuel de baking
                let renderer_map = map.to_render_tilemap().await;
                self.rendered_maps.insert(id.clone(), renderer_map);

                let layers = map.render_all_layers().await;
                self.rendered_layers.insert(id.clone(), layers);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            info!("WASM détecté : Le 'baking' de map est désactivé pour économiser la mémoire vidéo.");
            // On ne fait RIEN ici, les HashMaps 'rendered_maps' resteront vides.
        }
    }

    pub async fn finalize_textures(&self) {
        clear_background(BLACK);

        for spritesheet in self.spritesheets.values() {
            draw_texture_ex(
                &spritesheet.texture.clone(),
                -1000.0, -1000.0, 
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(1.0, 1.0)),
                    ..Default::default()
                },
            );
        }
        next_frame().await;
        next_frame().await;
    }

    // --- Logique de chargement principale ---
    pub async fn load_assets_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Récupération de l'URL de base depuis le JS (ou vide sur PC)
        let base_url = WebContext::get_base_url();
        info!("AssetServer Base URL: {}", base_url);

        // Note: 'path' ici est déjà résolu dans App::run, donc load_string fonctionnera
        let json_content = match load_string(path).await {
            Ok(s) => s,
            Err(e) => {
                error!("AssetServer: Failed to download/read file '{}'. Error: {}", path, e);
                return Err(Box::new(e));
            }
        };
        let asset_data: AssetFileData = serde_json::from_str(&json_content)?;

        // 2. Chargement des Maps (on résout le chemin)
        for map_data in asset_data.maps {
            let resolved_path = Self::resolve_path(&base_url, &map_data.path);
            info!("Loading Map: {} from {}", map_data.id, resolved_path);
            self.load_tiled_map(map_data.id, &resolved_path).await.unwrap();
        }

        // 3. Chargement des Spritesheets (on résout le chemin)
        for ss_data in asset_data.spritesheets {
            let resolved_path = Self::resolve_path(&base_url, &ss_data.path);
            info!("Loading Spritesheet: {} from {}", ss_data.id, resolved_path);

            let texture = match load_texture(&resolved_path).await {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to load texture {}. Error: {}", resolved_path, e);
                    continue; // Skip sans crasher
                }
            };

            texture.set_filter(FilterMode::Nearest);
            
            let sprite_width = texture.width() / ss_data.columns as f32;
            let sprite_height = texture.height() / ss_data.rows as f32;

            let spritesheet = Spritesheet::new(texture, sprite_width, sprite_height);
            self.add_spritesheet(ss_data.id, spritesheet);
        }

        // 4. Création des Animations (Pas besoin de path ici, c'est des IDs)
        for anim_data in asset_data.animations {
            // ... (code inchangé, logique pure) ...
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

        // 5. Chargement des fonts (on résout le chemin)
        for font_data in asset_data.fonts {
            let resolved_path = Self::resolve_path(&base_url, &font_data.path);
            info!("Loading Font: {} from {}", font_data.id, resolved_path);
            
            if let Ok(font) = load_ttf_font(&resolved_path).await {
                 self.fonts.insert(font_data.id.clone(), font);
            } else {
                error!("Failed to load font {}", resolved_path);
            }
        }

        // 6. Chargement des sons (on résout le chemin)
        for sound_data in asset_data.sounds {
            let resolved_path = Self::resolve_path(&base_url, &sound_data.path);
            info!("Loading Sound: {} from {}", sound_data.id, resolved_path);
            self.load_sound(&sound_data.id, &resolved_path).await;
        }

        Ok(())
    }

    pub fn render_layer(&self, map_id: &str, layer_name: &str, camera_rect: Option<Rect>) {
        
        // --- BRANCHE NATIF (PC) ---
        #[cfg(not(target_arch = "wasm32"))]
        {
            // On essaie de récupérer la texture pré-rendue
            if let Some(layers) = self.rendered_layers.get(map_id) {
                if let Some(render_target) = layers.get(layer_name) {
                    // On dessine simplement la texture géante
                    let texture = &render_target.texture;
                    draw_texture_ex(
                        texture,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams::default()
                    );
                    return; // On a fini pour le PC
                }
            }
        }

        // --- BRANCHE WEB (WASM) ---
        // Cette partie est compilée sur WASM, OU sur PC si le render target n'existe pas (fallback)
        self.render_layer_direct(map_id, layer_name, camera_rect);
    }

    /// Fonction interne pour le rendu tuile par tuile (Optimisé pour le Web)
    fn render_layer_direct(&self, map_id: &str, layer_name: &str, camera_rect: Option<Rect>) {
        let map = match self.maps.get(map_id) {
            Some(m) => m,
            None => return,
        };

        let layer_data = match map.tile_layers.get(layer_name) {
            Some(d) => d,
            None => return, // Layer introuvable ou invisible
        };

        let tile_w = map.tile_width as f32;
        let tile_h = map.tile_height as f32;

        // CALCUL DU CULLING (Zone visible)
        // On ne dessine que ce que la caméra voit. Très important pour les perfs en JS.
        let (min_x, min_y, max_x, max_y) = if let Some(cam) = camera_rect {
            (
                (cam.x / tile_w).floor() as i32,
                (cam.y / tile_h).floor() as i32,
                ((cam.x + cam.w) / tile_w).ceil() as i32,
                ((cam.y + cam.h) / tile_h).ceil() as i32,
            )
        } else {
            (0, 0, map.width as i32, map.height as i32)
        };

        // On s'assure de ne pas sortir des limites du tableau
        let start_x = min_x.max(0) as u32;
        let start_y = min_y.max(0) as u32;
        let end_x = (max_x as u32).min(map.width);
        let end_y = (max_y as u32).min(map.height);

        // Boucle de rendu optimisée
        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = (x + y * map.width) as usize;
                
                // Sécurité
                if idx >= layer_data.len() { continue; }
                
                let gid = layer_data[idx];
                
                // Si gid == 0, la tuile est vide
                if gid == 0 { continue; }

                // Trouver le bon tileset pour ce GID
                // On cherche le tileset dont le first_gid est <= au gid actuel
                if let Some(tileset) = map.tilesets.iter().rev().find(|ts| gid >= ts.first_gid) {
                    let local_id = gid - tileset.first_gid;
                    let sheet_cols = tileset.columns;
                    
                    // Coordonnées dans la spritesheet source
                    let tx = (local_id % sheet_cols) as f32 * tileset.tile_width;
                    let ty = (local_id / sheet_cols) as f32 * tileset.tile_height;

                    // Position monde
                    let dest_x = x as f32 * tile_w;
                    let dest_y = y as f32 * tile_h;

                    draw_texture_ex(
                        &tileset.spritesheet.texture,
                        dest_x,
                        dest_y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(tx, ty, tileset.tile_width, tileset.tile_height)),
                            dest_size: Some(vec2(tile_w, tile_h)),
                            ..Default::default()
                        }
                    );
                }
            }
        }
    }
}
