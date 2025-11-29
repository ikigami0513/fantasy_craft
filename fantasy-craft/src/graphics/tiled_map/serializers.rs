use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TiledMapData {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub tilesets: Vec<TilesetData>,
    pub layers: Vec<LayerData>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TilesetData {
    pub firstgid: u32,
    pub name: String,
    pub columns: u32,
    pub tilecount: u32,
    pub imagewidth: u32,
    pub imageheight: u32,
    pub image: String,
    #[serde(default)]
    pub tilewidth: u32, 
    #[serde(default)]
    pub tileheight: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(tag="type")]
pub enum LayerData {
    #[serde(rename = "tilelayer")]
    TileLayer {
        name: String,
        data: Vec<u32>,
        width: u32,
        height: u32,
        visible: bool
    },
    #[serde(other)]
    Other
}