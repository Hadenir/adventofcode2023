use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub(crate) enum EngineCell {
    Number(usize, u32),
    Symbol(usize, char),
    Empty,
}

impl EngineCell {
    pub fn get_number(&self) -> u32 {
        match self {
            EngineCell::Number(_, num) => *num,
            _ => panic!()
        }
    }
}

impl PartialEq for EngineCell {
    fn eq(&self, other: &Self) -> bool {
        use EngineCell::*;
        match (self, other) {
            (Empty, Empty) => true,
            (Number(id1, _), Number(id2, _)) => id1 == id2,
            (Symbol(id1, _), Symbol(id2, _)) => id1 == id2,
            _ => false,
        }
    }
}

impl Eq for EngineCell {}

impl PartialOrd for EngineCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EngineCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use EngineCell::*;
        match (self, other) {
            (Empty, Empty) => std::cmp::Ordering::Equal,
            (Empty, _) => std::cmp::Ordering::Less,
            (_, Empty) => std::cmp::Ordering::Greater,
            (Number(id1, _), Number(id2, _)) => id1.cmp(id2),
            (Number(id1, _), Symbol(id2, _)) => id1.cmp(id2),
            (Symbol(id1, _), Number(id2, _)) => id1.cmp(id2),
            (Symbol(id1, _), Symbol(id2, _)) => id1.cmp(id2),
        }
    }
}

pub(crate) struct EngineSchematic {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<EngineCell>,
}

impl EngineSchematic {
    pub fn get_cell(&self, x: usize, y: usize) -> EngineCell {
        self.cells[x + y * self.width]
    }

    pub fn get_surrounding(&self, x: usize, y: usize) -> impl IntoIterator<Item=EngineCell> {
        let mut cells = Vec::with_capacity(8);

        for j in y.saturating_sub(1)..=(y+1).min(self.height - 1) {
            for i in x.saturating_sub(1)..=(x+1).min(self.width - 1) {
                if i != x || j != y {
                    cells.push(self.get_cell(i, j));
                }
            }
        }

        cells
    }
}

impl FromStr for EngineSchematic {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines[0].len();
        let height = lines.len();

        let mut cells = Vec::with_capacity(width * height);
        let mut part_id = 0;

        for line in lines {
            let mut current_num = 0;
            for char in line.chars() {
                match char {
                    '0'..='9' => {
                        let digit = char.to_digit(10).unwrap();
                        current_num = current_num * 10 + digit;
                    }
                    _ if current_num > 0 => {
                        let len = current_num.ilog10() + 1;
                        for _ in 0..len {
                            cells.push(EngineCell::Number(part_id, current_num));
                        }
                        part_id += 1;
                        current_num = 0;
                    }
                    _ => (),
                }

                match char {
                    '.' => cells.push(EngineCell::Empty),
                    '0'..='9' => (),
                    _ => {
                        cells.push(EngineCell::Symbol(part_id, char));
                        part_id += 1;
                    }
                }
            }

            if current_num > 0 {
                let len = current_num.ilog10() + 1;
                for _ in 0..len {
                    cells.push(EngineCell::Number(part_id, current_num));
                }
                part_id += 1;
            }
        }

        assert_eq!(cells.len(), width * height);

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}
