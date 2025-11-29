use macroquad::prelude::*;
use crate::{physics::components::Transform, core::context::Context, graphics::tiled_map::components::{MainTileMap, TileMapComponent}};
use crate::camera::camera2d::components::{CameraTarget, CameraComponent, MainCamera};

pub fn update_camera(ctx: &mut Context) {
    // 1. Trouver la cible (inchangé)
    let mut target_position: Option<Vec2> = None;
    for (_entity, (transform, _)) in ctx.world.query::<(&Transform, &CameraTarget)>().iter() {
        target_position = Some(transform.position);
        break;
    }

    let Some(target_position) = target_position else {
        return;
    };

    // 2. Trouver la taille du monde (inchangé)
    let mut world_size: Option<Vec2> = None;
    for (_entity, (tilemap_comp, _main_map)) in ctx.world.query::<(&TileMapComponent, &MainTileMap)>().iter() {
        if let Some(map) = ctx.asset_server.get_map(&tilemap_comp.0) {
            world_size = Some(Vec2::new(
                map.width as f32 * map.tile_width as f32,
                map.height as f32 * map.tile_height as f32
            ));
        }
        break;
    }

    // 3. Mettre à jour la caméra
    for (_entity, (camera_comp, transform, _main)) in ctx.world.query::<(&mut CameraComponent, &mut Transform, &MainCamera)>().iter() {
        let lerp_speed = 1.0 - (-camera_comp.lerp_factor * ctx.dt()).exp();

        let mut desired_position = transform.position.lerp(target_position, lerp_speed);

        if let Some(world_size) = world_size {
            // Note: screen_width() / zoom donne la taille de la vue en unités monde
            // Attention : Macroquad gère le zoom bizarrement (1.0 = 1 pixel écran par unité monde si zoom non normalisé)
            // Ta formule de zoom ci-dessous normalise par rapport à l'écran, ce qui est bien.
            
            // Calcul approximatif de la demi-vue pour le clamping
            // (Peut nécessiter d'être ajusté selon si ta caméra est centrée ou non)
            let half_view_w = (screen_width() / camera_comp.zoom) * 0.5;
            let half_view_h = (screen_height() / camera_comp.zoom) * 0.5;

            desired_position.x = desired_position.x.clamp(half_view_w, world_size.x - half_view_w);
            desired_position.y = desired_position.y.clamp(half_view_h, world_size.y - half_view_h);
        }

        transform.position = desired_position;

        // MODIFICATION : Mise à jour de la caméra stockée dans le composant
        camera_comp.camera.target = transform.position;
        camera_comp.camera.rotation = 0.0;
        // On recalcule le zoom à chaque frame au cas où la fenêtre change de taille
        camera_comp.camera.zoom = vec2(
            camera_comp.zoom * 2.0 / screen_width(), 
            camera_comp.zoom * 2.0 / screen_height()
        );

        // On active la caméra pour le reste de la frame (ou jusqu'au prochain set_camera)
        set_camera(&camera_comp.camera);
    }
}
