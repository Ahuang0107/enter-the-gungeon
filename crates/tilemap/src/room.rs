use crate::layer::{EntityLayer, TilesLayer};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TilemapRoom {
    pub offset: (f32, f32),
    pub tiles_layers: Vec<TilesLayer>,
    pub entities_layers: Vec<EntityLayer>,
}

impl TilemapRoom {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            offset: (x, y),
            tiles_layers: vec![],
            entities_layers: vec![],
        }
    }
    pub fn push_tiles_layer(&mut self, layer: TilesLayer) {
        self.tiles_layers.push(layer);
    }
    pub fn push_entities_layer(&mut self, layer: EntityLayer) {
        self.entities_layers.push(layer);
    }
}
