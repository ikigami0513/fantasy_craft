use macroquad::prelude::*;
use crate::{components::Transform, context::Context, tiled_map::{MainTileMap, TileMapComponent}};

#[derive(Debug)]
pub struct Camera {
    pub lerp_factor: f32,
    pub zoom: f32
}

#[derive(Debug)]
pub struct MainCamera;

#[derive(Debug)]
pub struct CameraTarget;

pub fn update_camera(ctx: &mut Context) {
    let mut target_position: Option<Vec2> = None;
    for (_entity, (transform, _)) in ctx.world.query::<(&Transform, &CameraTarget)>().iter() {
        target_position = Some(transform.position);
        break;
    }

    let Some(target_position) = target_position else {
        return;
    };

    let mut world_size: Option<Vec2> = None;
    for (_entity, (tilemap_comp, _main_map)) in ctx.world.query::<(&TileMapComponent, &MainTileMap)>().iter() {
        if let Some(map) = ctx.asset_server.get_map(&tilemap_comp.name) {
            world_size = Some(Vec2::new(
                map.width as f32 * map.tile_width as f32,
                map.height as f32 * map.tile_height as f32
            ));
        }
        break;
    }

    for (_entity, (camera_comp, transform, _main)) in ctx.world.query::<(&mut Camera, &mut Transform, &MainCamera)>().iter() {
        let lerp_speed = 1.0 - (-camera_comp.lerp_factor * ctx.dt).exp();

        let mut desired_position = transform.position.lerp(target_position, lerp_speed);

        if let Some(world_size) = world_size {
            let half_view_w = screen_width() / (2.0 * camera_comp.zoom);
            let half_view_h = screen_height() / (2.0 * camera_comp.zoom);

            desired_position.x = desired_position.x.clamp(half_view_w, world_size.x - half_view_w);
            desired_position.y = desired_position.y.clamp(half_view_h, world_size.y - half_view_h);
        }

        transform.position = desired_position;

        let camera = Camera2D {
            target: transform.position,
            rotation: 0.0,
            zoom: vec2(camera_comp.zoom * 2.0 / screen_width(), camera_comp.zoom * 2.0 / screen_height()),
            ..Default::default()
        };
        set_camera(&camera);
    }
}
