use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct LevelModel {
    pub rooms: Vec<RoomModel>,
    pub tilesets: Vec<Tileset>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct RoomModel {
    /// 这里的坐标是根据room的左下角位置计算的
    /// 同时也需要注意room内的tile的坐标都是以这个点作为原点配置的
    pub world_pos: [i32; 2],
    pub size: [u32; 2],
    pub walls: Vec<TileGroup>,
    pub floors: Vec<TileGroup>,
    pub roofs: Vec<TileGroup>,
    pub lights: Vec<Light>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct TileGroup {
    pub tileset_uuid: String,
    pub tiles: HashMap<u32, HashMap<u32, u8>>,
}

impl TileGroup {
    pub fn insert(&mut self, grid_x: u32, grid_y: u32, index: u8) {
        if let Some(col) = self.tiles.get_mut(&grid_x) {
            col.insert(grid_y, index);
        } else {
            self.tiles.insert(grid_x, HashMap::from([(grid_y, index)]));
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Light {
    pub pos: [u32; 3],
    pub color: [u8; 4],
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct Tileset {
    pub uuid: String,
    pub src: String,
    pub tiles: HashMap<u8, ([u32; 2], [u32; 2])>,
    // 😔 因为wall的tile是倾斜显示的，所以创建对应的mesh时需要调整height的尺寸
    pub tilt: bool,
}
