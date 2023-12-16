
#[derive(Debug, Clone, Default, Hash)]
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
        if x < self.width && y < self.height {
            self.tiles.get(x + y * self.width)
        } else {
            None
        }
    }
}
