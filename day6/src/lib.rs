mod parser;

use parser::parse_input;

pub struct Race {
    pub time: u64,
    pub record_distance: u64,
}

fn count_winning(race: &Race) -> u64 {
    let t = race.time as f64;
    let d = race.record_distance as f64;
    let delta = t * t - 4.0 * d;
    let delta_sqrt = delta.sqrt();

    let b_0 = (t - delta_sqrt) / 2.0;
    let b_1 = (t + delta_sqrt) / 2.0;

    (b_1.ceil() as u64 - 1).saturating_sub(b_0.floor() as u64 + 1) + 1
}

pub fn solve_part_1(input: &str) -> u64 {
    let races = parse_input(input);

    races.into_iter().map(|race| count_winning(&race)).product()
}

pub fn solve_part_2(input: &str) -> u64 {
    let races = parse_input(input);

    let (time, dist): (String, String) = races
        .into_iter()
        .map(|race| (race.time, race.record_distance))
        .fold(
            (String::new(), String::new()),
            |(acc_time, acc_dist), (time, dist)| {
                (format!("{acc_time}{time}"), format!("{acc_dist}{dist}"))
            },
        );

    let race = Race {
        time: time.parse().unwrap(),
        record_distance: dist.parse().unwrap(),
    };

    count_winning(&race)
}
