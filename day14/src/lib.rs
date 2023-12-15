use std::collections::BTreeSet;

use crate::reflector_dish::{ReflectorDish, Tile};

mod grid;
mod reflector_dish;

pub fn solve_part_1(input: &str) -> u64 {
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

    dish.north_load()
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

        if let Some(j) = (y + 1..dish.grid.height)
            .take_while(|&j| dish.grid.get(x, j) == Some(&Tile::Empty))
            .last()
        {
            *dish.grid.get_mut(x, y).unwrap() = Tile::Empty;
            *dish.grid.get_mut(x, j).unwrap() = Tile::RoundedRock;
        }
    }
}

#[inline]
fn tilt_dish_east(dish: &mut ReflectorDish) {
    for (y, x) in dish.grid.iter_indices().rev() {
        if dish.grid.get(x, y) != Some(&Tile::RoundedRock) {
            continue;
        }

        if let Some(i) = (x + 1..dish.grid.width)
            .take_while(|&i| dish.grid.get(i, y) == Some(&Tile::Empty))
            .last()
        {
            *dish.grid.get_mut(x, y).unwrap() = Tile::Empty;
            *dish.grid.get_mut(i, y).unwrap() = Tile::RoundedRock;
        }
    }
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut dish: ReflectorDish = input.parse().unwrap();

    let hash_state = ahash::RandomState::new();
    let mut visited = BTreeSet::new();
    let mut cycle = Vec::new();

    let dish_hash = hash_state.hash_one(&dish);
    visited.insert(dish_hash);
    cycle.push((dish_hash, dish.north_load()));

    const NUM_CYCLES: usize = 1000000000;
    for i in 0..NUM_CYCLES {
        tilt_dish_north(&mut dish);
        tilt_dish_west(&mut dish);
        tilt_dish_south(&mut dish);
        tilt_dish_east(&mut dish);

        let dish_hash = hash_state.hash_one(&dish);
        if !visited.insert(dish_hash) {
            // Found cycle

            let start_idx = cycle
                .iter()
                .position(|&(hash, _)| hash == dish_hash)
                .expect("cycle exists");
            let end_idx = i + 1;
            let cycle_len = end_idx - start_idx;
            let idx = (NUM_CYCLES - start_idx) % cycle_len;
            return cycle[start_idx + idx].1;
        }

        cycle.push((dish_hash, dish.north_load()));
    }

    panic!("Did not found any cycle!")
}
