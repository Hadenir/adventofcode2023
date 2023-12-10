use std::fs;

use day10::*;

fn main() {
    let contents = fs::read_to_string("day10/input.txt").expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

const INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

const INPUT1A: &str = "S---7
|...|
|...|
|...|
L---J";


const INPUT3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ.F7FJ-
L---JF-JLJ....FJLJJ7
|F|F-JF---7...L7L|7|
|FFJF7L7F-JF7..L---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part_1() {
        let sol1 = solve_part_1(INPUT1);
        let sol2 = solve_part_1(INPUT2);

        assert_eq!(sol1, 4);
        assert_eq!(sol2, 8);
    }

    #[test]
    fn test_part_2() {
        let sol1 = solve_part_2(INPUT1);
        let sol1a = solve_part_2(INPUT1A);
        let sol2 = solve_part_2(INPUT2);
        let sol3 = solve_part_2(INPUT3);

        assert_eq!(sol1, 1);
        assert_eq!(sol1a, 9);
        assert_eq!(sol2, 1);
        assert_eq!(sol3, 10);
    }
}
