use crate::layout::Layout;

pub struct Level {
    width: usize,
    height: usize,
    layouts: Vec<Layout>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layouts: vec![],
        }
    }
}

#[cfg(test)]
mod test {}
