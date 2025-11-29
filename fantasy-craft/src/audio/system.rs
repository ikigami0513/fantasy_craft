use macroquad::audio::play_sound_once;
use crate::audio::event::PlaySoundEvent;
use crate::core::context::Context;
use crate::core::event::EventBus;

pub fn audio_system(ctx: &mut Context) {
    // 1. Split borrows (ResourceMap vs AssetServer stored in Context)
    // Assuming AssetServer is a field in Context, not in ResourceMap
    // If AssetServer is in ResourceMap, use: ctx.resources.get::<AssetServer>()
    
    let (event_bus_opt, asset_server) = (
        ctx.resources.get::<EventBus>(),
        &ctx.asset_server
    );

    if let Some(event_bus) = event_bus_opt {
        // 2. Read events
        for event in event_bus.read::<PlaySoundEvent>() {
            if let Some(sound) = asset_server.get_sound(&event.sound_name) {
                // Macroquad function to play sound
                play_sound_once(sound);
            } else {
                eprintln!("Warning: Sound not found: {}", event.sound_name);
            }
        }
    }
}
