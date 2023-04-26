/// ⚠️ 注意：生成的所有贴图位置信息都是tile的索引，其中`[0,0]`对应`[8.0,8.0]`的位置
///
/// 生成一个房间时，先生成对应的信息，再根据对应的信息往assets中加载贴图
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RoomModel {
    /// 相对世界的位置
    pos_index: [i32; 2],
    // TODO 暂时认为这里写死更方便，不需要灵活
    /// 使用墙壁贴图的位置，比较特殊因为tile size不是`[16.0,16.0]`而是`[16.0,32.0]`
    /// 但是位置依旧是用16tile计算出来的
    pub walls: Vec<SpriteModel>,
    pub floor: Vec<(String, Vec<SpriteModel>)>,
    /// 使用屋顶贴图的位置
    pub roofs: Vec<SpriteModel>,
    pub lights: Vec<LightModel>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpriteModel {
    /// 相对房间的位置，以中心为锚点
    pos_index: [i32; 2],
    tile_index: usize,
}

impl SpriteModel {
    pub const fn from_pos(x: i32, y: i32) -> Self {
        Self::from(x, y, 0)
    }
    pub const fn from(x: i32, y: i32, index: usize) -> Self {
        Self {
            pos_index: [x, y],
            tile_index: index,
        }
    }
    pub fn translation(&self) -> [f32; 2] {
        let x = self.pos_index[0] as f32;
        let y = self.pos_index[1] as f32;
        [(x * 16.0 + 8.0) / 10.0, (y * 16.0 + 8.0) / 10.0]
    }
    // TODO 这里比较特殊的是，wall是以两块tile为一个单位的，所以计算y的位置时还需要加上8.0
    pub fn wall_translation(&self) -> [f32; 2] {
        let x = self.pos_index[0] as f32;
        let y = self.pos_index[1] as f32;
        [(x * 16.0 + 8.0) / 10.0, (y * 16.0 + 8.0 + 8.0) / 10.0]
    }
    pub fn sprite_index(&self) -> usize {
        self.tile_index
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LightModel {
    pos_index: [i32; 2],
    pub color: [u8; 4],
}

impl LightModel {
    pub const fn from_pos(x: i32, y: i32) -> Self {
        Self {
            pos_index: [x, y],
            color: [255, 255, 255, 255],
        }
    }
    pub const fn from(x: i32, y: i32, color: [u8; 4]) -> Self {
        Self {
            pos_index: [x, y],
            color,
        }
    }
    pub fn translation(&self) -> [f32; 2] {
        let x = self.pos_index[0] as f32;
        let y = self.pos_index[1] as f32;
        [(x * 16.0 + 8.0) / 10.0, (y * 16.0 + 8.0) / 10.0]
    }
}

impl RoomModel {
    pub fn translation(&self) -> [f32; 2] {
        let x = self.pos_index[0] as f32;
        let y = self.pos_index[1] as f32;
        [x, y]
    }
    pub fn initial() -> Self {
        Self {
            pos_index: [0, 0],
            walls: vec![
                // 最上面的墙
                SpriteModel::from_pos(-10, 10),
                SpriteModel::from_pos(-9, 10),
                SpriteModel::from_pos(-8, 10),
                SpriteModel::from_pos(-7, 10),
                SpriteModel::from_pos(-6, 10),
                SpriteModel::from_pos(-5, 10),
                SpriteModel::from_pos(-4, 10),
                SpriteModel::from_pos(-3, 10),
                SpriteModel::from_pos(-2, 10),
                SpriteModel::from_pos(-1, 10),
                SpriteModel::from_pos(0, 10),
                SpriteModel::from_pos(1, 10),
                SpriteModel::from_pos(2, 10),
                SpriteModel::from_pos(3, 10),
                SpriteModel::from_pos(4, 10),
                SpriteModel::from_pos(5, 10),
                SpriteModel::from_pos(6, 10),
                SpriteModel::from_pos(7, 10),
                SpriteModel::from_pos(8, 10),
                SpriteModel::from_pos(9, 10),
                // 左边门口的墙
                SpriteModel::from_pos(-11, -4),
            ],
            floor: vec![
                // 地板
                (
                    String::from("Floor Brick"),
                    vec![
                        SpriteModel::from(-6, 5, 3),
                        SpriteModel::from(-6, 4, 3),
                        SpriteModel::from(-6, 3, 3),
                        SpriteModel::from(-6, 2, 3),
                        SpriteModel::from(-6, 1, 3),
                        SpriteModel::from(-6, 0, 3),
                        SpriteModel::from(-6, -1, 3),
                        SpriteModel::from(-6, -2, 3),
                        SpriteModel::from(-6, -3, 3),
                        SpriteModel::from(-6, -4, 3),
                        SpriteModel::from(-6, -5, 3),
                        //
                        SpriteModel::from(5, 5, 3),
                        SpriteModel::from(5, 4, 3),
                        SpriteModel::from(5, 3, 3),
                        SpriteModel::from(5, 2, 3),
                        SpriteModel::from(5, 1, 3),
                        SpriteModel::from(5, 0, 3),
                        SpriteModel::from(5, -1, 3),
                        SpriteModel::from(5, -2, 3),
                        SpriteModel::from(5, -3, 3),
                        SpriteModel::from(5, -4, 3),
                        SpriteModel::from(5, -5, 3),
                        //
                        SpriteModel::from(-5, 5, 3),
                        SpriteModel::from(-4, 5, 3),
                        SpriteModel::from(-3, 5, 3),
                        SpriteModel::from(-2, 5, 3),
                        SpriteModel::from(-1, 5, 3),
                        SpriteModel::from(0, 5, 3),
                        SpriteModel::from(1, 5, 3),
                        SpriteModel::from(2, 5, 3),
                        SpriteModel::from(3, 5, 3),
                        SpriteModel::from(4, 5, 3),
                        //
                        SpriteModel::from(-5, -5, 3),
                        SpriteModel::from(-4, -5, 3),
                        SpriteModel::from(-3, -5, 3),
                        SpriteModel::from(-2, -5, 3),
                        SpriteModel::from(-1, -5, 3),
                        SpriteModel::from(0, -5, 3),
                        SpriteModel::from(1, -5, 3),
                        SpriteModel::from(2, -5, 3),
                        SpriteModel::from(3, -5, 3),
                        SpriteModel::from(4, -5, 3),
                    ],
                ),
                // 地毯中心
                (
                    String::from("Carpet Blue"),
                    vec![
                        SpriteModel::from(0, 0, 11),
                        SpriteModel::from(-1, 0, 11),
                        SpriteModel::from(-2, 0, 17),
                        SpriteModel::from(1, 0, 17),
                        SpriteModel::from(-2, 1, 17),
                        SpriteModel::from(-1, 1, 17),
                        SpriteModel::from(0, 1, 17),
                        SpriteModel::from(1, 1, 17),
                        SpriteModel::from(-2, -1, 17),
                        SpriteModel::from(-1, -1, 17),
                        SpriteModel::from(0, -1, 17),
                        SpriteModel::from(1, -1, 17),
                        //
                        SpriteModel::from(-3, 3, 15),
                        SpriteModel::from(-3, 2, 15),
                        SpriteModel::from(-3, 1, 15),
                        SpriteModel::from(-3, 0, 15),
                        SpriteModel::from(-3, -1, 15),
                        SpriteModel::from(-3, -2, 15),
                        SpriteModel::from(-3, -3, 15),
                        SpriteModel::from(-4, 3, 15),
                        SpriteModel::from(-4, 2, 15),
                        SpriteModel::from(-4, 1, 15),
                        SpriteModel::from(-4, 0, 15),
                        SpriteModel::from(-4, -1, 15),
                        SpriteModel::from(-4, -2, 15),
                        SpriteModel::from(-4, -3, 15),
                        SpriteModel::from(2, 3, 15),
                        SpriteModel::from(2, 2, 15),
                        SpriteModel::from(2, 1, 15),
                        SpriteModel::from(2, 0, 15),
                        SpriteModel::from(2, -1, 15),
                        SpriteModel::from(2, -2, 15),
                        SpriteModel::from(2, -3, 15),
                        SpriteModel::from(3, 3, 15),
                        SpriteModel::from(3, 2, 15),
                        SpriteModel::from(3, 1, 15),
                        SpriteModel::from(3, 0, 15),
                        SpriteModel::from(3, -1, 15),
                        SpriteModel::from(3, -2, 15),
                        SpriteModel::from(3, -3, 15),
                        SpriteModel::from(-2, 3, 15),
                        SpriteModel::from(-2, 2, 15),
                        SpriteModel::from(-2, -2, 15),
                        SpriteModel::from(-2, -3, 15),
                        SpriteModel::from(-1, 3, 15),
                        SpriteModel::from(-1, 2, 15),
                        SpriteModel::from(-1, -2, 15),
                        SpriteModel::from(-1, -3, 15),
                        SpriteModel::from(0, 3, 15),
                        SpriteModel::from(0, 2, 15),
                        SpriteModel::from(0, -2, 15),
                        SpriteModel::from(0, -3, 15),
                        SpriteModel::from(1, 3, 15),
                        SpriteModel::from(1, 2, 15),
                        SpriteModel::from(1, -2, 15),
                        SpriteModel::from(1, -3, 15),
                        SpriteModel::from(-5, 4, 0),
                        SpriteModel::from(-4, 4, 1),
                        SpriteModel::from(-3, 4, 1),
                        SpriteModel::from(-2, 4, 1),
                        SpriteModel::from(-1, 4, 1),
                        SpriteModel::from(0, 4, 1),
                        SpriteModel::from(1, 4, 1),
                        SpriteModel::from(2, 4, 1),
                        SpriteModel::from(3, 4, 1),
                        SpriteModel::from(4, 4, 2),
                        SpriteModel::from(-5, -4, 12),
                        SpriteModel::from(-4, -4, 13),
                        SpriteModel::from(-3, -4, 13),
                        SpriteModel::from(-2, -4, 13),
                        SpriteModel::from(-1, -4, 13),
                        SpriteModel::from(0, -4, 13),
                        SpriteModel::from(1, -4, 13),
                        SpriteModel::from(2, -4, 13),
                        SpriteModel::from(3, -4, 13),
                        SpriteModel::from(4, -4, 14),
                        SpriteModel::from(-5, -3, 6),
                        SpriteModel::from(-5, -2, 6),
                        SpriteModel::from(-5, -1, 6),
                        SpriteModel::from(-5, 0, 6),
                        SpriteModel::from(-5, 1, 6),
                        SpriteModel::from(-5, 2, 6),
                        SpriteModel::from(-5, 3, 6),
                        SpriteModel::from(4, -3, 8),
                        SpriteModel::from(4, -2, 8),
                        SpriteModel::from(4, -1, 8),
                        SpriteModel::from(4, 0, 8),
                        SpriteModel::from(4, 1, 8),
                        SpriteModel::from(4, 2, 8),
                        SpriteModel::from(4, 3, 8),
                    ],
                ),
            ],
            // TODO 写到这里发现其实只按照固定规则来划分tileset得到的index可读性差
            //  而且tileset增加内容，修改排版时会导致index改变，不是很方便开发
            roofs: vec![
                // 最上面的天花板
                SpriteModel::from(-10, 12, 13),
                SpriteModel::from(-9, 12, 13),
                SpriteModel::from(-8, 12, 13),
                SpriteModel::from(-7, 12, 13),
                SpriteModel::from(-6, 12, 13),
                SpriteModel::from(-5, 12, 13),
                SpriteModel::from(-4, 12, 13),
                SpriteModel::from(-3, 12, 13),
                SpriteModel::from(-2, 12, 13),
                SpriteModel::from(-1, 12, 13),
                SpriteModel::from(0, 12, 13),
                SpriteModel::from(1, 12, 13),
                SpriteModel::from(2, 12, 13),
                SpriteModel::from(3, 12, 13),
                SpriteModel::from(4, 12, 13),
                SpriteModel::from(5, 12, 13),
                SpriteModel::from(6, 12, 13),
                SpriteModel::from(7, 12, 13),
                SpriteModel::from(8, 12, 13),
                SpriteModel::from(9, 12, 13),
                // 最下面的天花板
                // TODO 注意这里有个trick，靠近屏幕一侧的roof都比实际往下一格
                //  因为正常侧面的通道是两格，而墙高也是两格，那么侧面的通道的floor就正好被下方的roof盖住来
                SpriteModel::from(-10, -10, 1),
                SpriteModel::from(-9, -10, 1),
                SpriteModel::from(-8, -10, 1),
                SpriteModel::from(-7, -10, 1),
                SpriteModel::from(-6, -10, 1),
                SpriteModel::from(-5, -10, 1),
                SpriteModel::from(-4, -10, 1),
                SpriteModel::from(-3, -10, 1),
                SpriteModel::from(-2, -10, 1),
                SpriteModel::from(-1, -10, 1),
                SpriteModel::from(0, -10, 1),
                SpriteModel::from(1, -10, 1),
                SpriteModel::from(2, -10, 1),
                SpriteModel::from(3, -10, 1),
                SpriteModel::from(4, -10, 1),
                SpriteModel::from(5, -10, 1),
                SpriteModel::from(6, -10, 1),
                SpriteModel::from(7, -10, 1),
                SpriteModel::from(8, -10, 1),
                SpriteModel::from(9, -10, 1),
                // 最右边的天花板
                SpriteModel::from(10, 11, 6),
                SpriteModel::from(10, 10, 6),
                SpriteModel::from(10, 9, 6),
                SpriteModel::from(10, 8, 6),
                SpriteModel::from(10, 7, 6),
                SpriteModel::from(10, 6, 6),
                SpriteModel::from(10, 5, 6),
                SpriteModel::from(10, 4, 6),
                SpriteModel::from(10, 3, 6),
                SpriteModel::from(10, 2, 6),
                SpriteModel::from(10, 1, 6),
                SpriteModel::from(10, 0, 6),
                SpriteModel::from(10, -1, 6),
                SpriteModel::from(10, -2, 6),
                SpriteModel::from(10, -3, 6),
                SpriteModel::from(10, -4, 6),
                SpriteModel::from(10, -5, 6),
                SpriteModel::from(10, -6, 6),
                SpriteModel::from(10, -7, 6),
                SpriteModel::from(10, -8, 6),
                SpriteModel::from(10, -9, 6),
                // 最左边的天花板
                SpriteModel::from(-11, 11, 8),
                SpriteModel::from(-11, 10, 8),
                SpriteModel::from(-11, 9, 8),
                SpriteModel::from(-11, 8, 8),
                SpriteModel::from(-11, 7, 8),
                SpriteModel::from(-11, 6, 8),
                SpriteModel::from(-11, 5, 8),
                SpriteModel::from(-11, 4, 8),
                SpriteModel::from(-11, 3, 8),
                SpriteModel::from(-11, 2, 8),
                SpriteModel::from(-11, 1, 8),
                SpriteModel::from(-11, 0, 8),
                SpriteModel::from(-11, -1, 8),
                //
                SpriteModel::from(-11, -2, 14),
                // SpriteModel::from(-11, -3, 8),
                // SpriteModel::from(-11, -4, 8),
                // SpriteModel::from(-11, -5, 8),
                //
                SpriteModel::from(-11, -6, 2),
                SpriteModel::from(-11, -7, 8),
                SpriteModel::from(-11, -8, 8),
                SpriteModel::from(-11, -9, 8),
                // 左上角
                SpriteModel::from(-11, 12, 4),
                // 右上角
                SpriteModel::from(10, 12, 5),
                // 左下角
                SpriteModel::from(-11, -10, 10),
                // 右下角
                SpriteModel::from(10, -10, 11),
            ],
            lights: vec![
                LightModel::from_pos(-9, 10),
                LightModel::from_pos(8, 10),
                LightModel::from_pos(-10, -10),
                LightModel::from_pos(9, -10),
                // TODO 房间中直接照亮地板的光需要提升亮度到100000，高度到5，范围到50，颜色偏红(255,154,154)
                // LightModel::from(-2, 1, [255, 154, 154, 229]),
            ],
        }
    }
}
