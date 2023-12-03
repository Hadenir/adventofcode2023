mod engine_schematic;
mod parser;

use std::collections::BTreeSet;

use engine_schematic::EngineCell;
use parser::parse_input;

pub fn solve_part_1(input: &str) -> u32 {
    let schematic = parse_input(input);

    let mut set = BTreeSet::new();
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            let cell = schematic.get_cell(x, y);
            match cell {
                EngineCell::Number(_, _) if !set.contains(&cell) => {
                    if schematic
                        .get_surrounding(x, y)
                        .into_iter()
                        .any(|cell| matches!(cell, EngineCell::Symbol(_, _)))
                    {
                        set.insert(cell);
                    }
                }
                _ => (),
            }
        }
    }

    set.into_iter().map(|cell| cell.get_number()).sum()
}

pub fn solve_part_2(input: &str) -> u32 {
    let schematic = parse_input(input);

    let mut result = 0;
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            let cell = schematic.get_cell(x, y);
            if let EngineCell::Symbol(_, '*') = cell {
                let set: BTreeSet<_> = schematic
                    .get_surrounding(x, y)
                    .into_iter()
                    .filter(|cell| matches!(cell, EngineCell::Number(_, _)))
                    .collect();

                if set.len() == 2{
                    result += set.into_iter().map(|cell| cell.get_number()).product::<u32>();
                }
            }
        }
    }

    result
}
