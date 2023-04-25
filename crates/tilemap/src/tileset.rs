#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tileset {
    pub uid: usize,
    pub c_w: usize,
    pub c_h: usize,
    pub tile_grid_size: usize,
    pub rel_path: String,
}

impl From<&ldtk::TilesetDefinition> for Tileset {
    fn from(value: &ldtk::TilesetDefinition) -> Self {
        Self {
            uid: value.uid,
            c_w: value.c_wid,
            c_h: value.c_hei,
            tile_grid_size: value.tile_grid_size,
            rel_path: value.rel_path.clone(),
        }
    }
}
