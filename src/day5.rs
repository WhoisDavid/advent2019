use crate::{get_input, AdventError, AdventResult};

// Based on day 2

pub fn solve_part1() -> AdventResult<isize> {
    let code = get_input::<isize>(5)?.first_row();
    run_program(code, 1)
}

pub fn solve_part2() -> AdventResult<isize> {
    let code = get_input::<isize>(5)?.first_row();
    run_program(code, 5)
}

fn run_program(code: Vec<isize>, input: isize) -> AdventResult<isize> {
    let mut program = Program::new(code, input);
    program.run();
    program.output.pop().ok_or(AdventError::InvalidValue)
}

struct Program {
    program: Vec<isize>,
    instruction_pointer: usize,
    input: isize,
    output: Vec<isize>,
}

impl Program {
    fn new(program: Vec<isize>, input: isize) -> Self {
        Self {
            program,
            instruction_pointer: 0,
            input,
            output: Vec::new(),
        }
    }

    fn get_instruction(&self) -> isize {
        self.program[self.instruction_pointer]
    }

    fn get_param(&self, shift: usize) -> isize {
        self.program[self.instruction_pointer + shift]
    }

    fn get_position_param(&self, shift: usize) -> isize {
        self.program[self.program[self.instruction_pointer + shift] as usize]
    }

    fn get_params(&self, num_params: usize) -> Vec<isize> {
        let mut params_instruction = self.get_instruction() / 100;
        let mut params = vec![0; num_params];
        for (i, param) in params.iter_mut().enumerate() {
            let position_mode = (params_instruction % 10) == 0;
            if position_mode {
                *param = self.get_position_param(i + 1);
            } else {
                *param = self.get_param(i + 1);
            }
            params_instruction /= 10;
        }
        params
    }

    fn add(&mut self) {
        let params = self.get_params(2);
        let target = self.get_param(3) as usize;
        self.program[target] = params[0] + params[1];
        self.instruction_pointer += 4
    }

    fn mul(&mut self) {
        let params = self.get_params(2);
        let target = self.get_param(3) as usize;
        self.program[target] = params[0] * params[1];
        self.instruction_pointer += 4
    }

    fn input(&mut self) {
        let target = self.get_param(1) as usize;
        self.program[target] = self.input; // read!();
        self.instruction_pointer += 2
    }

    fn output(&mut self) {
        let target = self.get_params(1)[0];
        self.output.push(target);
        println!("Output: {}", target);
        self.instruction_pointer += 2;
    }

    fn jump_if_true(&mut self) {
        let params = self.get_params(2);
        if params[0] != 0 {
            self.instruction_pointer = params[1] as usize;
        } else {
            self.instruction_pointer += 3
        }
    }

    fn jump_if_false(&mut self) {
        let params = self.get_params(2);
        if params[0] == 0 {
            self.instruction_pointer = params[1] as usize;
        } else {
            self.instruction_pointer += 3
        }
    }

    fn less_than(&mut self) {
        let params = self.get_params(2);
        let target = self.get_param(3) as usize;
        self.program[target] = (params[0] < params[1]) as isize;
        self.instruction_pointer += 4
    }

    fn equals(&mut self) {
        let params = self.get_params(2);
        let target = self.get_param(3) as usize;
        self.program[target] = (params[0] == params[1]) as isize;
        self.instruction_pointer += 4;
    }

    fn run(&mut self) {
        let op = self.get_instruction() % 100;
        match op {
            1 => self.add(),
            2 => self.mul(),
            3 => self.input(),
            4 => self.output(),
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equals(),
            99 => return,
            _ => panic!("Unsupported Op!"),
        };
        self.run()
    }
}

#[test]
fn test_case_day5_equals_position() {
    let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(run_program(program.to_vec(), 8).ok(), Some(1));
    assert_eq!(run_program(program.to_vec(), 9).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 7).ok(), Some(0));
}

#[test]
fn test_case_day5_less_position() {
    let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(run_program(program.to_vec(), 8).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 9).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 7).ok(), Some(1));
}

#[test]
fn test_case_day5_equals_immediate() {
    let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    assert_eq!(run_program(program.to_vec(), 8).ok(), Some(1));
    assert_eq!(run_program(program.to_vec(), 9).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 7).ok(), Some(0));
}

#[test]
fn test_case_day5_less_immediate() {
    let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    assert_eq!(run_program(program.to_vec(), 8).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 9).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 7).ok(), Some(1));
}

#[test]
fn test_case_day5_jump_position() {
    let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    assert_eq!(run_program(program.to_vec(), 0).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 99).ok(), Some(1));
    assert_eq!(run_program(program.to_vec(), 11).ok(), Some(1));
}

#[test]
fn test_case_day5_jump_immediate() {
    let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    assert_eq!(run_program(program.to_vec(), 0).ok(), Some(0));
    assert_eq!(run_program(program.to_vec(), 99).ok(), Some(1));
    assert_eq!(run_program(program.to_vec(), 11).ok(), Some(1));
}

#[test]
fn test_case_day5_large() {
    let program = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    assert_eq!(run_program(program.to_vec(), 8).ok(), Some(1000));
    assert_eq!(run_program(program.to_vec(), 9).ok(), Some(1001));
    assert_eq!(run_program(program.to_vec(), 7).ok(), Some(999));
}
