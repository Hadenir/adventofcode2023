use std::fs;

use day16::*;

fn main() {
    let contents = fs::read_to_string("day16/input.txt").expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 46);
    }

    #[test]
    fn test_part_2() {
        let sol = solve_part_2(INPUT);

        assert_eq!(sol, 51);
    }
}
