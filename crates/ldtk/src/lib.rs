use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "jsonVersion")]
    pub json_version: String,
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
    #[serde(rename = "gridSize")]
    pub grid_size: usize,
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TilesetDefinition {
    pub uid: usize,
    #[serde(rename = "__cWid")]
    pub c_wid: usize,
    #[serde(rename = "__cHei")]
    pub c_hei: usize,
    #[serde(rename = "pxWid")]
    pub px_wid: usize,
    #[serde(rename = "pxHei")]
    pub px_hei: usize,
    #[serde(rename = "tileGridSize")]
    pub tile_grid_size: usize,
    #[serde(rename = "relPath")]
    pub rel_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "pxWid")]
    pub px_wid: usize,
    #[serde(rename = "pxHei")]
    pub px_hei: usize,
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
