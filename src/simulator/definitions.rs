#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Forward,
    Back
}

#[derive(Copy, Clone)]
pub enum Side {
    Left,
    Right
}

pub(crate) const BLOCK_SPACE_SIZE: usize = 16;

pub type BlockSpace = [[[u8; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE];
