
use crate::engine_schematic::EngineSchematic;

pub(crate) fn parse_input(input: &str) -> EngineSchematic {
    input.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::engine_schematic::EngineCell;

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "12..3
.4*..";

        let schematic = parse_input(input);

        assert_eq!(schematic.width, 5);
        assert_eq!(schematic.height, 2);
        assert_eq!(schematic.get_cell(0, 0), EngineCell::Number(0, 12));
        assert_eq!(schematic.get_cell(2, 0), EngineCell::Empty);
        assert_eq!(schematic.get_cell(2, 1), EngineCell::Symbol(3, '*'));

        let surrounding = schematic.get_surrounding(2, 1).into_iter().collect_vec();
        assert_eq!(surrounding.len(), 5);
        assert_eq!(surrounding[0], EngineCell::Number(0, 12));
        assert_eq!(surrounding[1], EngineCell::Empty);
        assert_eq!(surrounding[2], EngineCell::Empty);
        assert_eq!(surrounding[3], EngineCell::Number(2, 4));
        assert_eq!(surrounding[4], EngineCell::Empty);
    }
}
