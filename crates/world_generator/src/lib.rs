pub use model::{LevelModel, Rect, RoomModel, Tile, TileGroup, Tileset};

mod model;

impl LevelModel {
    pub fn from<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(
            std::fs::read_to_string(path)?.as_str(),
        )?)
    }
}
