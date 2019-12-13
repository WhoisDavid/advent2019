use advent2019::intcode::get_program_last_output;
use advent2019::{get_input, AdventResult};

fn main() -> AdventResult<()> {
    println!("Intcode diagnostic code: {}", solve_part1()?);
    println!("Intcode diagnostic code: {}", solve_part2()?);
    Ok(())
}

pub fn solve_part1() -> AdventResult<isize> {
    let code = &get_input::<isize>(5)?.first_row();
    let input = &[1];
    Ok(get_program_last_output(code, input))
}

pub fn solve_part2() -> AdventResult<isize> {
    let code = &get_input::<isize>(5)?.first_row();
    let input = &[5];
    Ok(get_program_last_output(code, input))
}

#[test]
fn test_case_day5_equals_position() {
    let program = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(get_program_last_output(program, &[8]), 1);
    assert_eq!(get_program_last_output(program, &[9]), 0);
    assert_eq!(get_program_last_output(program, &[7]), 0);
}

#[test]
fn test_case_day5_less_position() {
    let program = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(get_program_last_output(program, &[8]), 0);
    assert_eq!(get_program_last_output(program, &[9]), 0);
    assert_eq!(get_program_last_output(program, &[7]), 1);
}

#[test]
fn test_case_day5_equals_immediate() {
    let program = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
    assert_eq!(get_program_last_output(program, &[8]), 1);
    assert_eq!(get_program_last_output(program, &[9]), 0);
    assert_eq!(get_program_last_output(program, &[7]), 0);
}

#[test]
fn test_case_day5_less_immediate() {
    let program = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
    assert_eq!(get_program_last_output(program, &[8]), 0);
    assert_eq!(get_program_last_output(program, &[9]), 0);
    assert_eq!(get_program_last_output(program, &[7]), 1);
}

#[test]
fn test_case_day5_jump_position() {
    let program = &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    assert_eq!(get_program_last_output(program, &[0]), 0);
    assert_eq!(get_program_last_output(program, &[99]), 1);
    assert_eq!(get_program_last_output(program, &[11]), 1);
}

#[test]
fn test_case_day5_jump_immediate() {
    let program = &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    assert_eq!(get_program_last_output(program, &[0]), 0);
    assert_eq!(get_program_last_output(program, &[99]), 1);
    assert_eq!(get_program_last_output(program, &[11]), 1);
}

#[test]
fn test_case_day5_large() {
    let program = &[
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    assert_eq!(get_program_last_output(program, &[8]), 1000);
    assert_eq!(get_program_last_output(program, &[9]), 1001);
    assert_eq!(get_program_last_output(program, &[7]), 999);
}
