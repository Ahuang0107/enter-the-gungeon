use crate::layer::TilemapLayer;

#[derive(Debug)]
pub struct TilemapLevel {
    pub c_w: usize,
    pub c_h: usize,
    pub tile_size: usize,
    pub layers: Vec<TilemapLayer>,
}

impl TilemapLevel {
    pub fn new(c_w: usize, c_h: usize, tile_size: usize) -> Self {
        Self {
            c_w,
            c_h,
            tile_size,
            layers: vec![],
        }
    }
    pub fn push_layer(&mut self, layer: TilemapLayer) {
        assert_eq!(layer.c_w, self.c_w);
        assert_eq!(layer.c_h, self.c_h);
        self.layers.push(layer);
    }
}
