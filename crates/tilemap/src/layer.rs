#[derive(Debug)]
pub struct TilemapLayer {
    pub name: String,
    pub(crate) c_w: usize,
    pub(crate) c_h: usize,
    pub tiles: Vec<Vec<u8>>,
    pub tileset_uid: usize,
}

impl TilemapLayer {
    pub fn from_slice(tileset_uid: usize, tiles: &[&[u8]]) -> Self {
        Self {
            name: String::new(),
            c_w: tiles[0].len(),
            c_h: tiles.len(),
            tiles: tiles.to_vec().iter().map(|row| row.to_vec()).collect(),
            tileset_uid,
        }
    }
    pub fn from_vec(tileset_uid: usize, tiles: Vec<Vec<u8>>) -> Self {
        Self {
            name: String::new(),
            c_w: tiles[0].len(),
            c_h: tiles.len(),
            tiles,
            tileset_uid,
        }
    }
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }
    pub fn for_each<F: FnMut((usize, usize, u8))>(&self, mut op: F) {
        for y in 0..self.c_h {
            for x in 0..self.c_w {
                op((x, y, self.tiles[y][x]))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::TilemapLayer;

    const SAMPLE: &'static [&'static [u8]] = &[
        &[1, 1, 1, 1, 1, 1, 1, 1],
        &[1, 0, 0, 0, 0, 0, 0, 1],
        &[1, 0, 0, 0, 0, 2, 0, 1],
        &[1, 0, 2, 0, 0, 0, 0, 1],
        &[1, 0, 0, 0, 0, 0, 0, 1],
        &[1, 1, 1, 1, 1, 1, 1, 1],
    ];

    #[test]
    fn check_new() {
        let layer = TilemapLayer::from_slice(0, SAMPLE);
        assert_eq!(layer.c_w, 8);
        assert_eq!(layer.c_h, 6);
        assert_eq!(layer.tiles[2][5], 2);
    }

    #[test]
    fn check_for_each() {
        let layer = TilemapLayer::from_slice(0, SAMPLE);
        layer.for_each(|(x, y, v)| match (x, y) {
            (0, 0) => assert_eq!(v, 1),
            (1, 0) => assert_eq!(v, 1),
            (0, 1) => assert_eq!(v, 1),
            (1, 1) => assert_eq!(v, 0),
            (5, 2) => assert_eq!(v, 2),
            (2, 3) => assert_eq!(v, 2),
            _ => {}
        })
    }
}
