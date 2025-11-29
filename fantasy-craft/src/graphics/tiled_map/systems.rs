use macroquad::prelude::*;
use crate::prelude::{Context, TileMapLayerComponent};
use crate::graphics::tiled_map::components::TileMapComponent;
// Assure-toi que ce chemin correspond à ton architecture
use crate::camera::camera2d::components::{CameraComponent, MainCamera};

/// Fonction utilitaire pour calculer le rectangle visible par la caméra active.
/// Retourne un Rect couvrant tout l'écran en coordonnées Monde.
fn get_visible_world_rect(ctx: &mut Context) -> Rect {
    // 1. CHANGER LE TYPE ICI : On veut une Option de RÉFÉRENCE (&)
    let mut active_camera: Option<&Camera2D> = None;
    
    let mut query = ctx.world.query::<(&CameraComponent, &MainCamera)>();
    for (_, (cam_comp, _)) in query.iter() {
        // 2. AJOUTER UN '&' ICI : On emprunte la caméra au lieu de la voler
        active_camera = Some(&cam_comp.camera);
        break; 
    }

    // Le reste fonctionne exactement pareil, car on peut appeler des méthodes sur une référence
    if let Some(cam) = active_camera {
        let top_left = cam.screen_to_world(vec2(0.0, 0.0));
        let bottom_right = cam.screen_to_world(vec2(screen_width(), screen_height()));
        
        Rect::new(
            top_left.x, 
            top_left.y, 
            bottom_right.x - top_left.x, 
            bottom_right.y - top_left.y
        )
    } else {
        Rect::new(0.0, 0.0, screen_width(), screen_height())
    }
}

/// Système affichant une TileMap complète (tous les layers d'un coup).
pub fn tilemap_render_system(ctx: &mut Context) {
    let visible_rect = get_visible_world_rect(ctx);

    for (_, tileset_comp) in ctx.world.query::<&TileMapComponent>().iter() {
        let map_id = &tileset_comp.0;

        // --- OPTIMISATION NATIF (PC) ---
        // Si une texture complète de la map existe (générée par prepare_loaded_tiledmap sur PC),
        // on l'affiche en une seule passe. C'est le plus rapide pour le CPU.
        if let Some(rendered_map) = ctx.asset_server.get_renderer_map(map_id) {
            draw_texture_ex(
                &rendered_map.texture.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rendered_map.width, rendered_map.height)),
                    ..Default::default()
                }
            );
        } 
        // --- RENDU DYNAMIQUE (WEB / WASM) ---
        // Si pas de texture géante (désactivé sur WASM pour éviter le crash mémoire),
        // on itère sur les layers et on les dessine tuile par tuile avec culling.
        else if let Some(map) = ctx.asset_server.get_map(map_id) {
            // Note: Si l'ordre des layers est important, assure-toi de les avoir triés
            // ou d'avoir une liste ordonnée dans ta struct TileMap.
            for layer_name in map.tile_layers.keys() {
                ctx.asset_server.render_layer(
                    map_id, 
                    layer_name, 
                    Some(visible_rect) // On passe le culling !
                );
            }
        }
    }
}

/// Système affichant des layers individuels (pour le Y-Sort ou Z-Index).
pub fn tilemap_layer_render_system(ctx: &mut Context) {
    let visible_rect = get_visible_world_rect(ctx);

    for (_, layer_comp) in ctx.world.query::<&TileMapLayerComponent>().iter() {
        // La fonction render_layer de l'AssetServer gère déjà la logique :
        // - Si PC : Utilise la texture du layer pré-calculée.
        // - Si Web : Utilise le rendu direct tuile par tuile avec le Rect fourni.
        ctx.asset_server.render_layer(
            &layer_comp.tilemap_name,
            &layer_comp.layer_name,
            Some(visible_rect)
        );
    }
}
