#[derive(Debug, Clone, Default)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    tiles: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, tiles: impl IntoIterator<Item = T>) -> Self {
        let tiles: Vec<_> = tiles.into_iter().collect();

        assert_eq!(tiles.len(), width * height);

        Self {
            width,
            height,
            tiles,
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.tiles.get(x + y * self.width)
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.tiles.get_mut(x + y * self.width)
    }

    pub fn iter_indices(&self) -> IterIndices {
        IterIndices::new(self)
    }
}

pub struct IterIndices {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl IterIndices {
    fn new<T>(grid: &Grid<T>) -> Self {
        Self {
            x: 0,
            y: 0,
            width: grid.width,
            height: grid.height,
        }
    }
}

impl Iterator for IterIndices {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }

        let pair = (self.x, self.y);
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        Some(pair)
    }
}
