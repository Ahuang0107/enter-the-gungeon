use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct LevelModel {
    pub rooms: Vec<RoomModel>,
    pub tilesets: Vec<Tileset>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct RoomModel {
    /// 这里的坐标是根据room的左上角位置计算的
    /// 同时也需要注意room内的tile的坐标都是以这个点作为原点配置的
    pub world_pos: [f32; 2],
    pub walls: Vec<TileGroup>,
    pub floors: Vec<TileGroup>,
    pub roofs: Vec<TileGroup>,
    pub lights: Vec<Light>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct TileGroup {
    pub tileset_uuid: String,
    pub tiles: Vec<Tile>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Tile {
    pub pos: [f32; 2],
    pub index: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Light {
    pub pos: [f32; 3],
    pub color: [u8; 4],
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct Tileset {
    pub uuid: String,
    pub src: String,
    pub tiles: HashMap<u8, Rect>,
    // 😔 因为wall的tile是倾斜显示的，所以创建对应的mesh时需要调整height的尺寸
    // TODO 这里导致其他地方的逻辑都变复杂了，最好能找一个更好的解决方法
    pub tilt: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Rect {
    pub min: [f32; 2],
    pub size: [u32; 2],
}
