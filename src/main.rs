use genetic_programming::binary::{parse_bytes, Instr};
use genetic_programming::vm::structures::{RESULT_REGISTER, BlockSpace};
use genetic_programming::genetic::evaluation::evaluate_instructions;

fn main() {
    /*
        Program to build a line of blocks from the start (0, 1, 0) to the last position before the
        boundary (15, 1, 0). Place blocks above, keep going until no longer able to advance. Print
        the length of the line when done.

        res = true;
        num = 0;
        while res:
            placeUp();
            num += 1;
            res = forward();
        print(num);
     */
    let instr = parse_bytes(vec![
        Instr::SetValue as u8, 0, 0,
        Instr::PlaceUp as u8,
        Instr::Increment as u8, 0,
        Instr::Forward as u8,
        Instr::JumpNotZero as u8, 1, RESULT_REGISTER,
        Instr::PrintRegister as u8, 0,
    ]);

    /*
        Evaluation target: only the first half of the line built above should have been filled.
        The other half should have been empty (air).
     */
    let mut target = BlockSpace::default();
    for i in 0..8 {
        target[i][1][0] = 1;
    }

    println!("Evaluation score: {}", evaluate_instructions(&instr, &target));
}