use crate::vm::structures::{Direction, Side, BlockSpace, BLOCK_SPACE_SIZE};

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
    blocks: BlockSpace, // 3D array of bytes
    turtle: Turtle
}

impl Simulator {
    pub(crate) fn new() -> Self {
        Simulator {
            blocks: BlockSpace::default(),
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

    pub(crate) fn try_move(&mut self, dir: Direction) -> bool {
        let pos = self.turtle.get_adjacent(dir);
        if Self::pos_in_bounds(pos) {
            let Vector3(x, y, z) = pos;
            if self.blocks[x as usize][y as usize][z as usize] == 0 {
                self.turtle.shift(pos);
                true
            } else {
                false
            }
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
            let block = &mut (self.blocks[x as usize][y as usize][z as usize]);
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

    pub(crate) fn try_place(&mut self, dir: Direction) -> bool {
        self.try_change(dir, 1)
    }

    pub(crate) fn try_dig(&mut self, dir: Direction) -> bool {
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

    pub(crate) fn get_state_copy(&self) -> BlockSpace {
        self.blocks.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_valid() {
        let mut sim = Simulator::new();
        let res = sim.try_move(Direction::Forward);
        assert_eq!(sim.turtle.pos, Vector3(1, 0, 0));
        assert!(res);
    }

    #[test]
    fn movement_block() {
        let mut sim = Simulator::new();
        sim.blocks[1][0][0] = 1;
        let res = sim.try_move(Direction::Forward);
        assert_eq!(sim.turtle.pos, Vector3(0, 0, 0));
        assert!(!res);
    }

    #[test]
    fn movement_boundary() {
        let mut sim = Simulator::new();
        let res = sim.try_move(Direction::Back);
        assert_eq!(sim.turtle.pos, Vector3(0, 0, 0));
        assert!(!res);
    }

    #[test]
    fn place_block() {
        let mut sim = Simulator::new();
        let res = sim.try_place(Direction::Forward);
        assert_eq!(sim.blocks[1][0][0], 1);
        assert!(res);
    }

    #[test]
    fn place_overlap() {
        let mut sim = Simulator::new();
        sim.blocks[1][0][0] = 1;
        let res = sim.try_place(Direction::Forward);
        // operation is idempotent, should produce no change
        assert_eq!(sim.blocks[1][0][0], 1);
        assert!(!res);
    }

    #[test]
    fn place_boundary() {
        let mut sim = Simulator::new();
        let res = sim.try_place(Direction::Down);
        // operation is invalid, should produce no change
        assert_eq!(sim, Simulator::new());
        assert!(!res);
    }

    #[test]
    fn dig_block() {
        let mut sim = Simulator::new();
        sim.blocks[1][0][0] = 1;
        let res = sim.try_dig(Direction::Forward);
        assert_eq!(sim.blocks[1][0][0], 0);
        assert!(res);
    }

    #[test]
    fn dig_air() {
        let mut sim = Simulator::new();
        let res = sim.try_dig(Direction::Forward);
        // operation is idempotent, should produce no change
        assert_eq!(sim.blocks[1][0][0], 0);
        assert!(!res);
    }

    #[test]
    fn dig_boundary() {
        let mut sim = Simulator::new();
        let res = sim.try_dig(Direction::Down);
        // operation is invalid, should produce no change
        assert_eq!(sim, Simulator::new());
        assert!(!res);
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