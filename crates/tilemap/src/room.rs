use crate::layer::TilemapLayer;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TilemapRoom {
    pub offset: (f32, f32),
    pub layers: Vec<TilemapLayer>,
}

impl TilemapRoom {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            offset: (x, y),
            layers: vec![],
        }
    }
    pub fn push_layer(&mut self, layer: TilemapLayer) {
        self.layers.push(layer);
    }
}
