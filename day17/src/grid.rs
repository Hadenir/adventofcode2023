
#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    pub width: usize,
    pub height: usize,
    cells: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, cells: impl IntoIterator<Item = T>) -> Self {
        Self {
            width,
            height,
            cells: cells.into_iter().collect(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.cells[x + y * self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_returns_correct_values() {
        let cells = [1,2,3, 4,5,6, 7,8,9];
        let grid = Grid::new(3, 3, cells);

        assert_eq!(grid.get(0, 0), 1);
        assert_eq!(grid.get(1, 2), 8);
        assert_eq!(grid.get(2, 0), 3);
    }
}
