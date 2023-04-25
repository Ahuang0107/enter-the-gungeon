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

#[cfg(test)]
mod test {
    use crate::TilemapLayer;

    const SAMPLE: &'static [((usize, usize), usize)] = &[
        ((0, 0), 1),
        ((1, 0), 1),
        ((0, 1), 1),
        ((1, 1), 0),
        ((5, 2), 2),
        ((2, 3), 2),
    ];

    #[test]
    fn check_new() {
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
