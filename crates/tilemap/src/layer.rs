#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TilemapLayer {
    pub name: String,
    pub tiles: Vec<((usize, usize), usize)>,
    pub tileset_uid: usize,
}

impl TilemapLayer {
    pub fn from_slice(tileset_uid: usize, tiles: &[((usize, usize), usize)]) -> Self {
        Self {
            name: String::new(),
            tiles: tiles.to_vec(),
            tileset_uid,
        }
    }
    pub fn from_vec(tileset_uid: usize, tiles: Vec<((usize, usize), usize)>) -> Self {
        Self {
            name: String::new(),
            tiles,
            tileset_uid,
        }
    }
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }
    pub fn for_each<F: FnMut((usize, usize, usize))>(&self, mut op: F) {
        for ((x, y), index) in self.tiles.iter() {
            op((*x, *y, *index))
        }
    }
}
