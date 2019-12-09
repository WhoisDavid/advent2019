use crate::intcode::IntCode;
use crate::{get_input, AdventError, AdventResult};
use itertools::Itertools;

pub fn solve_part1() -> AdventResult<isize> {
    let code = &get_input::<isize>(7)?.first_row();
    let res = max_thrusters(code).ok_or(AdventError::InvalidValue)?;
    println!("Output: {}", res);
    Ok(res)
}

pub fn solve_part2() -> AdventResult<isize> {
    let code = &get_input::<isize>(7)?.first_row();
    let res = max_thrusters_feedback_loop(code).ok_or(AdventError::InvalidValue)?;
    println!("Output: {}", res);
    Ok(res)
}

fn run_amplifiers(program: &[isize], phases: &[isize]) -> isize {
    let mut input_signal = 0;
    let mut intcode = IntCode::new(program);
    for phase in phases {
        intcode = IntCode::new(program);
        intcode.run_till_halt(&[*phase, input_signal]);
        input_signal = intcode.last_output();
    }
    intcode.last_output()
}

fn max_thrusters(program: &[isize]) -> Option<isize> {
    (0..=4)
        .permutations(5)
        .map(|p| run_amplifiers(program, &p))
        .max()
}

pub fn run_amplifiers_feedback_loop(program: &[isize], phases: &[isize]) -> isize {
    let mut amp_a = IntCode::new(program);
    let mut amp_b = IntCode::new(program);
    let mut amp_c = IntCode::new(program);
    let mut amp_d = IntCode::new(program);
    let mut amp_e = IntCode::new(program);

    let input_signal = 0;
    let mut out_a = amp_a.run_till_output(&[phases[0], input_signal]);
    let mut out_b = amp_b.run_till_output(&[phases[1], out_a]);
    let mut out_c = amp_c.run_till_output(&[phases[2], out_b]);
    let mut out_d = amp_d.run_till_output(&[phases[3], out_c]);
    let mut out_e = amp_e.run_till_output(&[phases[4], out_d]);

    while !amp_e.has_halted() {
        out_a = amp_a.run_till_output(&[out_e]);
        out_b = amp_b.run_till_output(&[out_a]);
        out_c = amp_c.run_till_output(&[out_b]);
        out_d = amp_d.run_till_output(&[out_c]);
        out_e = amp_e.run_till_output(&[out_d]);
    }

    out_e
}

fn max_thrusters_feedback_loop(program: &[isize]) -> Option<isize> {
    (5..=9)
        .permutations(5)
        .map(|p| run_amplifiers_feedback_loop(program, &p))
        .max()
}

// TESTS
#[test]
fn test_day7_case1_amplifiers() {
    let prog = &[
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    assert_eq!(run_amplifiers(prog, &[4, 3, 2, 1, 0]), 43210);
}

#[test]
fn test_day7_case2_amplifiers() {
    let prog = &[
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    assert_eq!(run_amplifiers(prog, &[0, 1, 2, 3, 4]), 54321);
}

#[test]
fn test_day7_case3_amplifiers() {
    let prog = &[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    assert_eq!(run_amplifiers(prog, &[1, 0, 4, 3, 2]), 65210);
}

#[test]
fn test_day7_case1_max_thrusters() {
    let prog = &[
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    assert_eq!(max_thrusters(prog), Some(43210));
}

#[test]
fn test_day7_case2_max_thrusters() {
    let prog = &[
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    assert_eq!(max_thrusters(prog), Some(54321));
}

#[test]
fn test_day7_case3_max_thrusters() {
    let prog = &[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    assert_eq!(max_thrusters(prog), Some(65210));
}

#[test]
fn test_day7_case1_part2() {
    let prog = &[
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    assert_eq!(
        run_amplifiers_feedback_loop(prog, &[9, 8, 7, 6, 5]),
        139_629_729
    );
    assert_eq!(max_thrusters_feedback_loop(prog), Some(139_629_729));
}

#[test]
fn test_day7_case2_part2() {
    let prog = &[
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    assert_eq!(run_amplifiers_feedback_loop(prog, &[9, 7, 8, 5, 6]), 18216);
    assert_eq!(max_thrusters_feedback_loop(prog), Some(18216));
}
