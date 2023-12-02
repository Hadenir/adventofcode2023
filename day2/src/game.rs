#[derive(Debug)]
pub(crate) struct Game {
    pub id: u32,
    pub sets: Vec<CubeSet>,
}

impl Game {
    pub fn new(id: u32, sets: Vec<CubeSet>) -> Self {
        Self { id, sets }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub(crate) struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}
