mod parser;

use std::{fmt::Display, iter};

use itertools::Itertools;
use parser::parse_input;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = &'static str;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        let spring = match char {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => return Err("Invalid character encountered"),
        };

        Ok(spring)
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        };

        write!(f, "{}", char)
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

// fn print(springs: &[Spring], groups: &[usize], left_in_group: Option<usize>) {
//     for s in springs {
//         print!("{}", s);
//     }
//     print!(" ");
//     if let Some(left_in_group) = left_in_group {
//         print!("{},", left_in_group);
//     }
//     for g in groups {
//         print!("{},", g);
//     }
//     println!();
// }

/// How many valid configurations of springs there are that satisfy specified groups.
fn count_valid_arrangements(
    springs: &[Spring],
    groups: &[usize],
    mut left_in_group: Option<usize>,
) -> usize {
    if springs.is_empty() {
        if groups.is_empty() && left_in_group.map(|x| x == 0).unwrap_or(true) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut next_group_idx = 0;
    for spring_idx in 0..springs.len() {
        match springs[spring_idx] {
            Spring::Operational => {
                if left_in_group.map(|x| x > 0).unwrap_or_default() {
                    return 0;
                } else {
                    left_in_group = None;
                    continue;
                }
            }
            Spring::Damaged => {
                if let Some(ref mut left_in_group) = left_in_group {
                    if *left_in_group == 0 {
                        return 0;
                    } else {
                        *left_in_group -= 1;
                        continue;
                    }
                } else if next_group_idx < groups.len() {
                    left_in_group = Some(groups[next_group_idx] - 1);
                    next_group_idx += 1;
                    continue;
                } else {
                    return 0;
                }
            }
            Spring::Unknown => {
                // Assume it's damaged.
                let mut num_confs = 0;

                if let Some(ref mut left_in_group) = left_in_group {
                    if *left_in_group > 0 {
                        let new_left_in_group = *left_in_group - 1;
                        let new_groups: Vec<_> = groups[next_group_idx..].into();

                        num_confs += count_valid_arrangements(
                            &springs[spring_idx + 1..],
                            &new_groups,
                            Some(new_left_in_group),
                        );
                    }
                } else if next_group_idx < groups.len() {
                    let new_left_in_group: usize = groups[next_group_idx] - 1;
                    let new_groups: Vec<_> = groups[next_group_idx + 1..].into();

                    num_confs += count_valid_arrangements(
                        &springs[spring_idx + 1..],
                        &new_groups,
                        Some(new_left_in_group),
                    );
                }

                // If it cannot be damaged, assume it's operational.
                if !left_in_group.map(|x| x > 0).unwrap_or_default() {
                    num_confs += count_valid_arrangements(
                        &springs[spring_idx + 1..],
                        &groups[next_group_idx..],
                        None,
                    );
                }

                return num_confs;
            }
        }
    }

    if next_group_idx == groups.len() && left_in_group.map(|x| x == 0).unwrap_or(true) {
        1
    } else {
        0
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let records = parse_input(input);

    records
        .into_iter()
        .map(|record| count_valid_arrangements(&record.springs, &record.groups, None))
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let records = parse_input(input)
        .into_iter()
        .map(|record| {
            let springs = iter::repeat(iter::once(Spring::Unknown).chain(record.springs))
                .take(5)
                .flatten()
                .skip(1)
                .collect();
            let groups = iter::repeat(record.groups).take(5).flatten().collect();

            Record { springs, groups }
        })
        .collect_vec();

    records
        .into_par_iter()
        .map(|record| {
            dbg!(count_valid_arrangements(
                &record.springs,
                &record.groups,
                None
            ))
        })
        .sum()
}
