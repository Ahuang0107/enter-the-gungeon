use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "jsonVersion")]
    pub json_version: String,
    #[serde(rename = "defaultGridSize")]
    pub default_grid_size: usize,
    pub defs: Definitions,
    pub levels: Vec<Level>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Definitions {
    pub layers: Vec<LayerDefinition>,
    pub tilesets: Vec<TilesetDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayerDefinition {
    pub uid: usize,
    pub identifier: String,
    #[serde(rename = "gridSize")]
    pub grid_size: usize,
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TilesetDefinition {
    pub uid: usize,
    pub identifier: String,
    #[serde(rename = "__cWid")]
    pub c_wid: usize,
    #[serde(rename = "__cHei")]
    pub c_hei: usize,
    #[serde(rename = "tileGridSize")]
    pub tile_grid_size: usize,
    #[serde(rename = "relPath")]
    pub rel_path: String,
}

impl TilesetDefinition {
    /// 根据 grid tiles 中的 src 得到在 tileset 中的 index
    /// index 从 1 开始，因为 0 被用来表示 null
    pub fn get_index(&self, src: (usize, usize)) -> usize {
        let (x, y) = src;
        assert!(x <= (self.c_wid - 1) * self.tile_grid_size);
        assert!(y <= (self.c_hei - 1) * self.tile_grid_size);
        let x_i = x / self.tile_grid_size;
        let y_i = y / self.tile_grid_size;
        x_i + 1 + y_i * self.c_wid
    }
}

#[cfg(test)]
mod test {
    use crate::TilesetDefinition;

    #[test]
    fn check() {
        let tileset = TilesetDefinition {
            uid: 0,
            identifier: String::new(),
            c_wid: 10,
            c_hei: 2,
            tile_grid_size: 16,
            rel_path: String::new(),
        };
        assert_eq!(tileset.get_index((0, 0)), 1);
        assert_eq!(tileset.get_index((16, 0)), 2);
        assert_eq!(tileset.get_index((32, 0)), 3);
        assert_eq!(tileset.get_index((32, 0)), 3);
        assert_eq!(tileset.get_index((144, 0)), 10);
        assert_eq!(tileset.get_index((0, 16)), 11);
        assert_eq!(tileset.get_index((16, 16)), 12);
        assert_eq!(tileset.get_index((144, 16)), 20);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    pub identifier: String,
    #[serde(rename = "pxWid")]
    pub px_wid: usize,
    #[serde(rename = "pxHei")]
    pub px_hei: usize,
    #[serde(rename = "worldX")]
    pub world_x: usize,
    #[serde(rename = "worldY")]
    pub world_y: usize,
    #[serde(rename = "layerInstances")]
    pub layer_instances: Vec<LayerInstance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayerInstance {
    #[serde(rename = "__identifier")]
    pub identifier: String,
    #[serde(rename = "__cWid")]
    pub c_wid: usize,
    #[serde(rename = "__cHei")]
    pub c_hei: usize,
    #[serde(rename = "__gridSize")]
    pub grid_size: usize,
    #[serde(rename = "__tilesetRelPath")]
    pub tileset_rel_path: String,
    #[serde(rename = "layerDefUid")]
    pub layer_def_uid: usize,
    #[serde(rename = "gridTiles")]
    pub grid_tiles: Vec<GridTile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridTile {
    /// 表示在layer中的位置，根据tile的左上角定位
    pub px: (usize, usize),
    /// 表示在tileset中的位置，根据tile的左上角定位
    pub src: (usize, usize),
}
