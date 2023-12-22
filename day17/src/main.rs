use std::fs;

use day17::*;

fn main() {
    let contents = fs::read_to_string("day17/input.txt").expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const INPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 102);
    }

    #[test]
    fn test_part_2() {
        let sol = solve_part_2(INPUT);
        let sol2 = solve_part_2(INPUT2);

        assert_eq!(sol, 94);
        assert_eq!(sol2, 71);
    }
}
