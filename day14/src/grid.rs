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
    back_x: usize,
    back_y: usize,
    width: usize,
}

impl IterIndices {
    fn new<T>(grid: &Grid<T>) -> Self {
        Self {
            x: 0,
            y: 0,
            back_x: grid.width - 1,
            back_y: grid.height - 1,
            width: grid.width,
        }
    }
}

impl Iterator for IterIndices {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.back_y && self.x > self.back_x {
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

impl DoubleEndedIterator for IterIndices {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y >= self.back_y && self.x > self.back_x {
            return None;
        }

        let pair = (self.back_x, self.back_y);
        if self.back_x == 0 {
            self.back_x = self.width - 1;
            self.back_y -= 1;
        }

        Some(pair)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let tiles = vec![0; 4];
        let grid = Grid::new(2, 2, tiles);

        let mut iter = grid.iter_indices();
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_back_iterator() {
        let tiles = vec![0; 4];
        let grid = Grid::new(2, 2, tiles);

        let mut iter = grid.iter_indices();
        assert_eq!(iter.next_back(), Some((1, 1)));
        assert_eq!(iter.next_back(), Some((0, 1)));
        assert_eq!(iter.next_back(), Some((1, 0)));
        assert_eq!(iter.next_back(), Some((0, 0)));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_double_endediterator() {
        let tiles = vec![0; 4];
        let grid = Grid::new(2, 2, tiles);

        let mut iter = grid.iter_indices();
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next_back(), Some((1, 1)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next_back(), Some((0, 1)));
        assert_eq!(iter.next(), None);
    }
}
