#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tileset {
    pub identifier: String,
    pub path: String,
    pub columns: usize,
    pub rows: usize,
    pub tile_size: (f32, f32),
}

impl From<&ldtk::TilesetDefinition> for Tileset {
    fn from(value: &ldtk::TilesetDefinition) -> Self {
        Self {
            identifier: value.identifier.clone(),
            path: value.rel_path.clone(),
            columns: value.c_wid,
            rows: value.c_hei,
            tile_size: (value.tile_grid_size as f32, value.tile_grid_size as f32),
        }
    }
}
