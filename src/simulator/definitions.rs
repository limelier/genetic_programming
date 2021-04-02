use rand::prelude::Distribution;
use rand::distributions::Standard;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Forward,
    Back
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Forward,
            _ => Direction::Back,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Side {
    Left,
    Right
}

impl Distribution<Side> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
        match rng.gen_range(0..=1) {
            0 => Side::Left,
            _ => Side::Right,
        }
    }
}

pub(crate) const BLOCK_SPACE_SIZE: usize = 16;

pub type BlockSpace = [[[u8; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE];
