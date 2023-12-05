mod almanac;
mod parser;
mod range_map;

use almanac::AlmanacItem;
use itertools::Itertools;
use parser::parse_input;

use crate::almanac::Seed;

pub fn solve_part_1(input: &str) -> u64 {
    let (
        seeds,
        seed_to_soil,
        soild_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse_input(input);

    seeds
        .into_iter()
        .map(|seed| seed_to_soil.get(seed))
        .map(|soil| soild_to_fertilizer.get(soil))
        .map(|fertilizer| fertilizer_to_water.get(fertilizer))
        .map(|water| water_to_light.get(water))
        .map(|light| light_to_temperature.get(light))
        .map(|temperature| temperature_to_humidity.get(temperature))
        .map(|humidity| humidity_to_location.get(humidity))
        .map(|location| location.value())
        .min()
        .unwrap()
}

pub fn solve_part_2(input: &str) -> u64 {
    let (
        seeds,
        seed_to_soil,
        soild_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse_input(input);

    println!("Input parsed");

    let seeds = seeds
        .into_iter()
        .tuples()
        .flat_map(|(start, len)| (start.value()..(start.value() + len.value())).map(Seed::new))
        .collect_vec();

    println!("Seeds generated");

    seeds
        .into_iter()
        .map(|seed| seed_to_soil.get(seed))
        .map(|soil| soild_to_fertilizer.get(soil))
        .map(|fertilizer| fertilizer_to_water.get(fertilizer))
        .map(|water| water_to_light.get(water))
        .map(|light| light_to_temperature.get(light))
        .map(|temperature| temperature_to_humidity.get(temperature))
        .map(|humidity| humidity_to_location.get(humidity))
        .map(|location| location.value())
        .min()
        .unwrap()
}
