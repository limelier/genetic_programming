use crate::vm::structures::{Direction, Side};

const BLOCK_SPACE_SIZE: usize = 16;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Orientation {
    /// Z+ goes south, X+ goes east
    ///    Z-
    ///  X-  X+
    ///    Z+
    XPos,
    XNeg,
    ZPos,
    ZNeg
}



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Vector3(isize, isize, isize);

impl Vector3 {
    fn add(&self, other: Self) -> Self {
        Vector3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

/// Get a Vector3 versor for the given direction, and if dir = Forward, given orientation.
fn delta(dir: Direction, facing: Orientation) -> Vector3 {
    match dir {
        Direction::Up => Vector3(0, 1, 0),
        Direction::Down => Vector3(0, -1, 0),
        Direction::Forward => match facing {
            Orientation::XPos => Vector3(1, 0, 0),
            Orientation::XNeg => Vector3(-1, 0, 0),
            Orientation::ZPos => Vector3(0, 0, 1),
            Orientation::ZNeg => Vector3(0, 0, -1)
        },
        Direction::Back => match facing {
            Orientation::XPos => Vector3(-1, 0, 0),
            Orientation::XNeg => Vector3(1, 0, 0),
            Orientation::ZPos => Vector3(0, 0, -1),
            Orientation::ZNeg => Vector3(0, 0, 1),
        }
    }
}

#[derive (Eq, PartialEq, Debug)]
struct Turtle {
    pos: Vector3,
    facing: Orientation
}

impl Turtle {
    fn get_adjacent(&self, dir: Direction) -> Vector3 {
        let delta = delta(dir, self.facing);
        self.pos.add(delta)
    }

    fn turn(&mut self, side: Side) {
        self.facing = match side {
            Side::Right => match self.facing {
                Orientation::XPos => Orientation::ZPos,
                Orientation::XNeg => Orientation::ZNeg,
                Orientation::ZPos => Orientation::XNeg,
                Orientation::ZNeg => Orientation::XPos,
            }
            Side::Left => match self.facing {
                Orientation::XPos => Orientation::ZNeg,
                Orientation::XNeg => Orientation::ZPos,
                Orientation::ZPos => Orientation::XPos,
                Orientation::ZNeg => Orientation::XNeg,
            }
        }
    }

    fn shift(&mut self, pos: Vector3) {
        self.pos = pos;
    }
}

#[derive (Eq, PartialEq, Debug)]
pub(crate) struct Simulator {
    blocks: [[[u8; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE], // 3D array of bytes
    turtle: Turtle
}

impl Simulator {
    pub(crate) fn new() -> Self {
        Simulator {
            blocks: [[[0; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE]; BLOCK_SPACE_SIZE],
            turtle: Turtle {
                pos: Vector3(0, 0, 0),
                facing: Orientation::XPos,
            },
        }
    }

    fn pos_in_bounds(pos: Vector3) -> bool {
        pos.0 >= 0 && pos.0 < BLOCK_SPACE_SIZE as isize &&
        pos.1 >= 0 && pos.1 < BLOCK_SPACE_SIZE as isize &&
        pos.2 >= 0 && pos.2 < BLOCK_SPACE_SIZE as isize
    }

    pub(crate) fn shift(&mut self, dir: Direction) -> bool {
        let pos = self.turtle.get_adjacent(dir);
        if Self::pos_in_bounds(pos) {
            self.turtle.shift(pos);
            true
        } else {
            false
        }
    }

    pub(crate) fn turn(&mut self, side: Side) {
        self.turtle.turn(side);
    }

    fn try_change(&mut self, dir: Direction, to: u8) -> bool {
        let pos = self.turtle.get_adjacent(dir);
        if Self::pos_in_bounds(pos) {
            let Vector3(x, y, z) = pos;
            let mut block = &mut (self.blocks[x as usize][y as usize][z as usize]);
            if *block != to {
                *block = to;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(crate) fn place(&mut self, dir: Direction) -> bool {
        self.try_change(dir, 1)
    }

    pub(crate) fn destroy(&mut self, dir: Direction) -> bool {
        self.try_change(dir, 0)
    }

    /// Check if the given direction is blocked. Out-of-bounds is considered blocked.
    pub(crate) fn detect(&self, dir: Direction) -> bool {
        let pos = self.turtle.get_adjacent(dir);
        if Self::pos_in_bounds(pos) {
            let Vector3(x, y, z) = pos;
            self.blocks[x as usize][y as usize][z as usize] != 0
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_valid() {
        let mut sim = Simulator::new();
        sim.shift(Direction::Forward);
        assert_eq!(sim.turtle.pos, Vector3(1, 0, 0));
        sim.shift(Direction::Up);
        assert_eq!(sim.turtle.pos, Vector3(1, 1, 0));
        sim.shift(Direction::Down);
        assert_eq!(sim.turtle.pos, Vector3(1, 0, 0));
        sim.shift(Direction::Back);
        assert_eq!(sim.turtle.pos, Vector3(0, 0, 0));
    }

    #[test]
    fn movement_invalid() {
        let mut sim = Simulator::new();
        sim.shift(Direction::Back);
        sim.shift(Direction::Down);
        assert_eq!(sim.turtle.pos, Vector3(0, 0, 0));
    }

    #[test]
    fn place_block() {
        let mut sim = Simulator::new();
        sim.place(Direction::Forward);
        assert_eq!(sim.blocks[1][0][0], 1);

        // idempotent: placing over another block does nothing
        sim.place(Direction::Forward);
        assert_eq!(sim.blocks[1][0][0], 1);
    }

    #[test]
    fn place_out_of_bounds() {
        let mut sim = Simulator::new();
        sim.place(Direction::Down);
        assert_eq!(sim, Simulator::new());
    }

    #[test]
    fn break_block() {
        let mut sim = Simulator::new();
        sim.blocks[1][0][0] = 1;
        sim.destroy(Direction::Forward);
        assert_eq!(sim.blocks[1][0][0], 0);

        // idempotent: breaking air does nothing
        sim.destroy(Direction::Forward);
        assert_eq!(sim.blocks[1][0][0], 0);
    }

    #[test]
    fn detect_air() {
        let sim = Simulator::new();
        assert!(!sim.detect(Direction::Forward));
    }

    #[test]
    fn detect_block() {
        let mut sim = Simulator::new();
        sim.blocks[1][0][0] = 1;
        assert!(sim.detect(Direction::Forward));
    }
    #[test]
    fn detect_out_of_bounds() {
        let sim = Simulator::new();
        assert!(sim.detect(Direction::Down));
    }
    #[test]
    fn turn() {
        let mut sim = Simulator::new();
        sim.turn(Side::Left);
        assert_eq!(sim.turtle.facing, Orientation::ZNeg);
        sim.turn(Side::Right);
        assert_eq!(sim.turtle.facing, Orientation::XPos);
    }
}