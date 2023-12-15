mod parser;

use itertools::Itertools;
use parser::parse_input;

fn hash_string(string: &str) -> u8 {
    string.chars().fold(0, |hash, char| {
        hash.wrapping_add(char as u8).wrapping_mul(17)
    })
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
    box_no: u8,
}

impl Lens {
    fn new(label: impl ToString, focal_length: u8) -> Self {
        let label = label.to_string();
        let box_no = hash_string(&label);

        Self {
            label,
            focal_length,
            box_no,
        }
    }

    fn by_box_no(&self) -> u8 {
        self.box_no
    }
}

#[derive(Debug)]
enum Instruction<'a> {
    RemoveLens { label: &'a str },
    InsertLens { label: &'a str, focal_length: u8 },
}

pub fn solve_part_1(input: &str) -> u64 {
    input.trim().split(',').map(|x| hash_string(x) as u64).sum()
}

pub fn solve_part_2(input: &str) -> u64 {
    let instructions = parse_input(input);

    let mut lens_boxes = Vec::<Lens>::new();
    for instruction in instructions {
        match instruction {
            Instruction::InsertLens {
                label,
                focal_length,
            } => match lens_boxes.iter_mut().find(|lens| lens.label == label) {
                Some(lens) => lens.focal_length = focal_length,
                None => {
                    let lens = Lens::new(label, focal_length);
                    lens_boxes.push(lens);
                    lens_boxes.sort_by_key(Lens::by_box_no);
                }
            },
            Instruction::RemoveLens { label } => {
                if let Some(pos) = lens_boxes.iter().position(|lens| lens.label == label) {
                    lens_boxes.remove(pos);
                }
            }
        }
    }

    lens_boxes
        .into_iter()
        .group_by(Lens::by_box_no)
        .into_iter()
        .flat_map(|(box_no, lenses)| {
            lenses.enumerate().map(move |(slot_no, lens)| {
                (box_no as u64 + 1) * (slot_no as u64 + 1) * lens.focal_length as u64
            })
        })
        .sum()
}
