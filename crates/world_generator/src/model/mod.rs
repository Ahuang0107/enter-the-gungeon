use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct LevelModel {
    pub rooms: Vec<RoomModel>,
    pub tilesets: Vec<Tileset>,
}

impl LevelModel {
    pub fn contains(&self, pos: [i32; 2]) -> bool {
        self.rooms.iter().find(|room| room.contains(pos)).is_some()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct RoomModel {
    /// 这里的坐标是根据room的左上角位置计算的
    /// 同时也需要注意room内的tile的坐标都是以这个点作为原点配置的
    pub world_pos: [i32; 2],
    pub size: [i32; 2],
    pub walkable_area: Vec<Rect>,
    // TODO 是否需要使用chunk优化
    // pub walkable_area_chunk: HashMap<(i32, i32, i32, i32), Vec<Rect>>,
    pub walls: Vec<TileGroup>,
    pub floors: Vec<TileGroup>,
    pub roofs: Vec<TileGroup>,
    pub lights: Vec<Light>,
}

impl RoomModel {
    pub fn get_rect(&self) -> Rect {
        Rect {
            min: self.world_pos,
            size: self.size,
        }
    }
    pub fn contains(&self, pos: [i32; 2]) -> bool {
        return if self.in_room(pos) {
            let pos = [pos[0] - self.world_pos[0], pos[1] - self.world_pos[1]];
            self.walkable_area
                .iter()
                .find(|area| area.contains(pos))
                .is_some()
        } else {
            false
        };
    }
    /// 只是用来判断某个点是否在room范围内，缩小需要判断walkable area的room范围
    fn in_room(&self, pos: [i32; 2]) -> bool {
        let x = pos[0];
        let y = pos[1];
        let min = self.world_pos;
        let max = [
            self.world_pos[0] + self.size[0],
            self.world_pos[1] - self.size[1],
        ];
        x >= min[0] && x <= max[0] && y <= min[1] && y >= max[1]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct TileGroup {
    pub tileset_uuid: String,
    pub tiles: Vec<Tile>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Tile {
    pub pos: [i32; 2],
    pub index: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Light {
    pub pos: [i32; 3],
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
    pub min: [i32; 2],
    pub size: [i32; 2],
}

impl Rect {
    pub fn get_max(&self) -> [i32; 2] {
        self.get_right_bottom()
    }
    pub fn get_left_top(&self) -> [i32; 2] {
        self.min
    }
    pub fn get_left_bottom(&self) -> [i32; 2] {
        [self.min[0], self.min[1] - self.size[1]]
    }
    pub fn get_right_top(&self) -> [i32; 2] {
        [self.min[0] + self.size[0], self.min[1]]
    }
    pub fn get_right_bottom(&self) -> [i32; 2] {
        [self.min[0] + self.size[0], self.min[1] - self.size[1]]
    }
    /// this is only for crate it self, you need to call RoomModel::contains when detect collision
    fn contains(&self, pos: [i32; 2]) -> bool {
        pos[0] >= self.min[0]
            && pos[0] <= self.get_max()[0]
            && pos[1] <= self.min[1]
            && pos[1] >= self.get_max()[1]
    }
}

#[cfg(test)]
mod test;
