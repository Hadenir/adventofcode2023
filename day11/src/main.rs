use std::fs;

use day11::*;

fn main() {
    let contents = fs::read_to_string("day11/input.txt").expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 374);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT, 2), 374);
        assert_eq!(solve_part_2(INPUT, 10), 1030);
        assert_eq!(solve_part_2(INPUT, 100), 8410);
    }
}
