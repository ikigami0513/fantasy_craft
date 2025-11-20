use hecs::Entity;

use crate::prelude::ComponentLoader;

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlignmentType {
    Left,
    Center,
    Right
}

impl HorizontalAlignmentType {
    pub fn to_str(&self) -> &'static str {
        match self {
            HorizontalAlignmentType::Left => "left",
            HorizontalAlignmentType::Center => "center",
            HorizontalAlignmentType::Right => "right"
        }
    }

    pub fn from_str(value: &str) -> HorizontalAlignmentType {
        match value {
            "left" => HorizontalAlignmentType::Left,
            "center" => HorizontalAlignmentType::Center,
            "right" => HorizontalAlignmentType::Right,
            _ => HorizontalAlignmentType::Left
        }
    }
}

#[derive(Debug, Clone)]
pub struct HorizontalAlignment(pub HorizontalAlignmentType);

pub struct HorizontalAlignmentLoader;

impl ComponentLoader for HorizontalAlignmentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = HorizontalAlignment(HorizontalAlignmentType::from_str(&loader_data));

        ctx.world.insert_one(entity, component).expect("Failed to insert HorizontalAlignment");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlignmentType {
    Top,
    Center,
    Bottom
}

impl VerticalAlignmentType {
    pub fn to_str(&self) -> &'static str {
        match self {
            VerticalAlignmentType::Top => "top",
            VerticalAlignmentType::Center => "center",
            VerticalAlignmentType::Bottom => "bottom"
        }
    }

    pub fn from_str(value: &str) -> VerticalAlignmentType {
        match value {
            "top" => VerticalAlignmentType::Top,
            "center" => VerticalAlignmentType::Center,
            "bottom" => VerticalAlignmentType::Bottom,
            _ => VerticalAlignmentType::Top
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerticalAlignment(pub VerticalAlignmentType);

pub struct VerticalAlignmentLoader;

impl ComponentLoader for VerticalAlignmentLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: Entity, data: &serde_json::Value) {
        let loader_data: String = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let component = VerticalAlignment(VerticalAlignmentType::from_str(&loader_data));

        ctx.world.insert_one(entity, component).expect("Failed to insert VerticalAlignment");
    }
}
