use hecs::Entity;
use serde::Deserialize;

use crate::prelude::{ComponentLoader};
use crate::gui::gui_dimension::{GuiDimension, GuiDimensionLoaderData};

#[derive(Debug, Clone, Copy, Default)]
pub struct GuiLocalOffset {
    pub x: GuiDimension,
    pub y: GuiDimension
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiLocalOffsetLoaderData {
    pub x: GuiDimensionLoaderData,
    pub y: GuiDimensionLoaderData
}

pub struct GuiLocalOffsetLoader;

impl ComponentLoader for GuiLocalOffsetLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let loader_data: GuiLocalOffsetLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let parse_dimension = |loader_dim: GuiDimensionLoaderData| -> GuiDimension {
            match loader_dim {
                GuiDimensionLoaderData::Pixels(px) => GuiDimension::Pixels(px),
                GuiDimensionLoaderData::Percent(s) => {
                    let value = s.trim_end_matches('%')
                                 .parse::<f32>()
                                 .unwrap_or(0.0); // 0% par défaut
                    GuiDimension::Percent(value / 100.0) 
                }
            }
        };

        let component = GuiLocalOffset {
            x: parse_dimension(loader_data.x),
            y: parse_dimension(loader_data.y)
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiLocalOffset");
    }
}
