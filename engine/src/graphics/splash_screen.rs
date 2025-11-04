use macroquad::prelude::*;
use crate::prelude::{Context, GuiBox, GuiImage, Transform};

#[derive(Debug)]
pub struct SplashScreenTag;

pub fn setup_splash_screen(ctx: &mut Context) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let logo_w = 400.0;
    let logo_h = 400.0;

    let pos_x = (screen_w / 2.0) - (logo_w / 2.0);
    let pos_y = (screen_h / 2.0) - (logo_h / 2.0);

    ctx.world.spawn((
        Transform {
            position: vec2(pos_x, pos_y),
            ..Default::default()
        },
        GuiBox {
            width: logo_w,
            height: logo_h,
            color: BLACK,
            ..Default::default()
        },
        GuiImage {
            texture: Some("logo_engine".to_string()),
            col_row: uvec2(0, 0),
            tint: WHITE,
            ..Default::default()
        },
        SplashScreenTag
    ));
}

pub fn despawn_splash_screen(ctx: &mut Context) {
    let mut entities_to_despawn = Vec::new();

    for (entity, _) in ctx.world.query::<&SplashScreenTag>().iter() {
        entities_to_despawn.push(entity);
    }

    for entity in entities_to_despawn {
        ctx.world.despawn(entity).expect("Failed to despawn splash entity");
    }
}