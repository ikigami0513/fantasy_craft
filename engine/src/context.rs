use hecs::World;

use crate::asset_server::AssetServer;

pub struct Context {
    pub world: World,
    pub asset_server: AssetServer,
    pub dt: f32
}
