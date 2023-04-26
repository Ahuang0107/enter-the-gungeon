#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TilemapLayer {
    pub tile_size: (f32, f32),
    columns: usize,
    rows: usize,
    pub tiles: Vec<(usize, usize, usize)>,
    pub tileset_identifier: String,
}

impl TilemapLayer {
    pub fn new(
        tile_size: (f32, f32),
        columns: usize,
        rows: usize,
        tiles: Vec<(usize, usize, usize)>,
        tileset_identifier: &str,
    ) -> Self {
        Self {
            tile_size,
            columns,
            rows,
            tiles,
            tileset_identifier: tileset_identifier.to_string(),
        }
    }
    pub fn for_each<F: FnMut((usize, usize, usize))>(&self, mut op: F) {
        for (x, y, index) in self.tiles.iter() {
            op((*x, *y, *index))
        }
    }
}
