pub enum LayoutType {
    Floor,
    FloorDecor,
    Wall,
    Roof,
    Shadow,
}

pub struct Layout {
    width: usize,
    height: usize,
    grids: Vec<u8>,
    index: usize,
}

impl Layout {
    pub fn new(grids: &[u8], width: Option<usize>) -> Self {
        let len = grids.len();
        let width = width.unwrap_or_else(|| {
            let width = (len as f32).sqrt() as usize;
            assert_eq!(width * width, len);
            width
        });
        let height = len / width;
        assert_eq!(height * width, len);
        Self {
            width,
            height,
            grids: grids.to_vec(),
            index: 0,
        }
    }
}

impl Iterator for Layout {
    type Item = (usize, usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grids.len() {
            return None;
        }
        let tile = self.grids[self.index];
        let x = self.index % self.width;
        let y = (self.index - x) / self.width;
        self.index += 1;
        Some((x, y, tile))
    }
}

#[cfg(test)]
mod test {
    use crate::layout::Layout;

    static GRIDS: &[u8] = &[1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1];

    #[test]
    fn check_new() {
        let layout = Layout::new(GRIDS, None);
        assert_eq!(layout.width, 4);
        assert_eq!(layout.height, 4);
    }

    #[test]
    fn check_foreach() {
        let layout = Layout::new(GRIDS, None);
        let mut e_x = 0;
        let mut e_y = 0;
        layout.for_each(|(x, y, tile)| {
            assert_eq!(x, e_x);
            assert_eq!(y, e_y);
            if y == 0 || y == 3 || ((y == 1 || y == 2) && (x == 0 || x == 3)) {
                assert_eq!(tile, 1);
            } else {
                assert_eq!(tile, 0);
            }
            if e_x < 3 {
                e_x += 1;
            } else {
                e_x = 0;
                e_y += 1;
            }
        });
    }
}
