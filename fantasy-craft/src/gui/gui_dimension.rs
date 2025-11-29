use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub enum GuiDimension {
    Pixels(f32),
    Percent(f32)
}

impl GuiDimension {
    pub fn resolve(&self, screen_dimension: f32) -> f32 {
        match self {
            GuiDimension::Pixels(px) => *px,
            GuiDimension::Percent(pct) => (*pct * screen_dimension).round()
        }
    }
}

impl Default for GuiDimension {
    fn default() -> Self {
        GuiDimension::Pixels(100.0)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GuiDimensionLoaderData {
    Pixels(f32),
    Percent(String),
}

impl Default for GuiDimensionLoaderData {
    fn default() -> Self {
        GuiDimensionLoaderData::Pixels(100.0)
    }
}