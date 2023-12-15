use itertools::Itertools;

use crate::reflector_dish::{ReflectorDish, Tile};

mod grid;
mod reflector_dish;

pub fn solve_part_1(input: &str) -> usize {
    let mut dish: ReflectorDish = input.parse().unwrap();

    for (x, y) in dish.grid.iter_indices() {
        if dish.grid.get(x, y) != Some(&Tile::RoundedRock) {
            continue;
        }

        if let Some(j) = (0..y)
            .rev()
            .take_while(|&j| dish.grid.get(x, j) == Some(&Tile::Empty))
            .last()
        {
            *dish.grid.get_mut(x, y).unwrap() = Tile::Empty;
            *dish.grid.get_mut(x, j).unwrap() = Tile::RoundedRock;
        }
    }

    dish.grid
        .iter_indices()
        .map(|(x, y)| match dish.grid.get(x, y).unwrap() {
            Tile::RoundedRock => dish.grid.height - y,
            _ => 0,
        })
        .sum()
}

#[inline]
fn tilt_dish_north(dish: &mut ReflectorDish) {
    for (x, y) in dish.grid.iter_indices() {
        if dish.grid.get(x, y) != Some(&Tile::RoundedRock) {
            continue;
        }

        if let Some(j) = (0..y)
            .rev()
            .take_while(|&j| dish.grid.get(x, j) == Some(&Tile::Empty))
            .last()
        {
            *dish.grid.get_mut(x, y).unwrap() = Tile::Empty;
            *dish.grid.get_mut(x, j).unwrap() = Tile::RoundedRock;
        }
    }
}

#[inline]
fn tilt_dish_west(dish: &mut ReflectorDish) {
    for (y, x) in dish.grid.iter_indices() {
        if dish.grid.get(x, y) != Some(&Tile::RoundedRock) {
            continue;
        }

        if let Some(i) = (0..x)
            .rev()
            .take_while(|&i| dish.grid.get(i, y) == Some(&Tile::Empty))
            .last()
        {
            *dish.grid.get_mut(x, y).unwrap() = Tile::Empty;
            *dish.grid.get_mut(i, y).unwrap() = Tile::RoundedRock;
        }
    }
}

#[inline]
fn tilt_dish_south(dish: &mut ReflectorDish) {
    for (x, y) in dish.grid.iter_indices().rev() {
        if dish.grid.get(x, y) != Some(&Tile::RoundedRock) {
            continue;
        }

        if let Some(j) = (0..y)
            .rev()
            .take_while(|&j| dish.grid.get(x, j) == Some(&Tile::Empty))
            .last()
        {
            *dish.grid.get_mut(x, y).unwrap() = Tile::Empty;
            *dish.grid.get_mut(x, j).unwrap() = Tile::RoundedRock;
        }
    }
}

pub fn solve_part_2(input: &str) -> usize {
    let mut dish: ReflectorDish = input.parse().unwrap();

    const NUM_CYCLES: usize = 1000000000;
    for i in 0..NUM_CYCLES {

        if i % 100000 == 0 {
            println!("{}/{}", i, NUM_CYCLES);
        }
    }

    dish.grid
        .iter_indices()
        .map(|(x, y)| match dish.grid.get(x, y).unwrap() {
            Tile::RoundedRock => dish.grid.height - y,
            _ => 0,
        })
        .sum()
}
