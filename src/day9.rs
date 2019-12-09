use crate::intcode::run_program;
use crate::{get_input, AdventResult};

pub fn solve_part1() -> AdventResult<Vec<isize>> {
    let code = &get_input::<isize>(9)?.first_row();
    let res = run_program(code, &[1]);
    println!("Output: {:?}", res);
    Ok(res)
}

pub fn solve_part2() -> AdventResult<Vec<isize>> {
    let code = &get_input::<isize>(9)?.first_row();
    let res = run_program(code, &[2]);
    println!("Output: {:?}", res);
    Ok(res)
}

#[test]
fn test_case_day9_quining() {
    let program = &[
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    assert_eq!(run_program(program, &[]), program);
}

#[test]
fn test_case_day9_16_digits() {
    let program = &[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
    let res = run_program(program, &[]).last().unwrap() / 1_000_000_000_000_000;
    assert!(0 < res && res < 10);
}

#[test]
fn test_case_day9_big_number() {
    let program = &[104, 1_125_899_906_842_624, 99];
    assert_eq!(run_program(program, &[]), vec![1_125_899_906_842_624]);
}
