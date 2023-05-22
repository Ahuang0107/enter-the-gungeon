use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct LevelModel {
    /// default brith point (in grid) in the level
    ///
    /// ex: brith point = `[-5,10]`, grid size = 16
    /// represent the actual brith point is `[-80.0,160.0]`
    pub brith_point: [i32; 2],
    pub rooms: Vec<RoomModel>,
    pub tilesets: Vec<Tileset>,
}

impl LevelModel {
    pub fn contains_floor(&self, grid_pos: [i32; 2]) -> bool {
        for room in self.rooms.iter() {
            if room.contains_floor(grid_pos) {
                return true;
            }
        }
        false
    }
    /// 判断pos所在的tile类型
    pub fn pos_tile(&self, grid_pos: [i32; 2]) -> Option<TileType> {
        for room in self.rooms.iter() {
            if let Some(type_) = room.pos_tile(grid_pos) {
                return Some(type_);
            }
        }
        None
    }
}

#[derive(Eq, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Roof,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct RoomModel {
    /// 这里的坐标是根据room的左下角位置计算的
    /// 同时也需要注意room内的tile的坐标都是以这个点作为原点配置的
    pub display_name: String,
    pub world_pos: [i32; 2],
    pub size: [u32; 2],
    pub walls: Vec<TileGroup>,
    pub floors: Vec<TileGroup>,
    pub roofs: Vec<TileGroup>,
    pub lights: Vec<Light>,
}

impl RoomModel {
    /// 判断pos所在的tile类型
    pub fn pos_tile(&self, grid_pos: [i32; 2]) -> Option<TileType> {
        let rel_grid_x = grid_pos[0] - self.world_pos[0];
        let rel_grid_y = grid_pos[1] - self.world_pos[1];
        if rel_grid_x < 0 || rel_grid_y < 0 {
            return None;
        }
        let rel_grid_pos = [rel_grid_x as u32, rel_grid_y as u32];
        if rel_grid_pos[0] > self.size[0] || rel_grid_pos[1] > self.size[1] {
            return None;
        }
        for tile_group in self.roofs.iter() {
            if tile_group.contains(rel_grid_pos) {
                return Some(TileType::Roof);
            }
        }
        for tile_group in self.walls.iter() {
            if tile_group.contains(rel_grid_pos) {
                return Some(TileType::Wall);
            }
        }
        for tile_group in self.floors.iter() {
            if tile_group.contains(rel_grid_pos) {
                return Some(TileType::Floor);
            }
        }
        None
    }
    pub fn contains_floor(&self, grid_pos: [i32; 2]) -> bool {
        let rel_grid_x = grid_pos[0] - self.world_pos[0];
        let rel_grid_y = grid_pos[1] - self.world_pos[1];
        if rel_grid_x < 0 || rel_grid_y < 0 {
            return false;
        }
        let rel_grid_pos = [rel_grid_x as u32, rel_grid_y as u32];
        if rel_grid_pos[0] > self.size[0] || rel_grid_pos[1] > self.size[1] {
            return false;
        }
        for tile_group in self.floors.iter() {
            if tile_group.contains(rel_grid_pos) {
                return true;
            }
        }
        false
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct TileGroup {
    pub tileset_uuid: String,
    pub tiles: HashMap<u32, HashMap<u32, u8>>,
}

impl TileGroup {
    pub fn contains(&self, grid_pos: [u32; 2]) -> bool {
        if let Some(col) = self.tiles.get(&grid_pos[0]) {
            if col.get(&grid_pos[1]).is_some() {
                return true;
            }
        }
        false
    }
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
