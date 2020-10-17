use genetic_programming::vm::program::Program;
use genetic_programming::binary::{parse_bytes, Instr};

fn main() {
    /*
        r0 = 1;
        while (r0 < 100) {
            print(r0);
            r0 += 10;
        }
     */
    let instr = parse_bytes(vec![
        Instr::SetValue as u8, 0, 1,
        Instr::CompareValue as u8, 0, 100,
        Instr::JumpNotLess as u8, 6,
        Instr::PrintRegister as u8, 0,
        Instr::AddValue as u8, 0, 10,
        Instr::Jump as u8, 1,
        Instr::Pass as u8,
    ]);
    let mut program = Program::from_instructions(&instr);

    program.execute();
}