use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

pub fn run_program(code: &[isize], input: &[isize]) -> Vec<isize> {
    let mut program = IntCode::new(code);
    program.run_till_halt(input);
    program.output
}

pub fn get_program_last_output(code: &[isize], input: &[isize]) -> isize {
    let mut program = IntCode::new(code);
    program.run_till_halt(input);
    program.last_output()
}

pub fn run_program_iteration(code: &[isize], input: &[isize]) -> isize {
    let mut program = IntCode::new(code);
    program.run_till_output(input)
}

#[derive(Clone)]
pub struct IntCode {
    memory: HashMap<usize, isize>,
    relative_base: isize,
    instruction_pointer: usize,
    input: VecDeque<isize>,
    output: Vec<isize>,
    program_halted: bool,
}

impl IntCode {
    pub fn new(program: &[isize]) -> Self {
        let mut computer = Self {
            memory: HashMap::new(),
            relative_base: 0,
            instruction_pointer: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            program_halted: false,
        };
        computer.set_program(program);
        computer
    }

    pub fn set_program(&mut self, program: &[isize]) {
        for (idx, instr) in program.iter().enumerate() {
            self.memory.insert(idx, *instr);
        }
    }

    pub fn set_input(&mut self, input: &[isize]) {
        self.input
            .append(&mut VecDeque::from_iter(input.to_owned()));
    }

    pub fn last_output(&self) -> isize {
        self.output.last().copied().expect("No output!")
    }

    pub fn has_halted(&self) -> bool {
        self.program_halted
    }

    pub fn run_till_halt(&mut self, input: &[isize]) {
        self.set_input(input);
        while !self.has_halted() {
            self.run_instruction()
        }
    }

    pub fn run_till_input(&mut self, input: &[isize]) -> &[isize] {
        self.set_input(input);
        self.run_instruction();
        let input_op = 3;
        while self.get_instruction() % 100 != input_op && !self.has_halted() {
            self.run_instruction();
        }
        &self.output
    }

    pub fn run_till_output(&mut self, input: &[isize]) -> isize {
        self.set_input(input);
        let output_op = 4;
        while self.get_instruction() % 100 != output_op {
            self.run_instruction();
        }
        self.run_instruction();
        self.last_output()
    }

    fn run_instruction(&mut self) {
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
            9 => self.set_relative_base(),
            99 => self.halt(),
            _ => panic!("Unsupported Op!"),
        };
    }

    fn read(&self, loc: usize) -> isize {
        match self.memory.get(&loc) {
            Some(m) => *m,
            None => 0,
        }
    }

    fn write(&mut self, loc: usize, val: isize) {
        self.memory.insert(loc, val);
    }

    fn get_instruction(&self) -> isize {
        self.read(self.instruction_pointer)
    }

    fn parameter_address(&self, shift: usize) -> usize {
        self.instruction_pointer + shift
    }

    fn position_parameter_address(&self, shift: usize) -> usize {
        self.read(self.instruction_pointer + shift) as usize
    }

    fn relative_parameter_address(&self, shift: usize) -> usize {
        let relative_mode_param = self.read(self.instruction_pointer + shift);
        (self.relative_base + relative_mode_param) as usize
    }

    fn parameters_address(&self, num_params: usize) -> Vec<usize> {
        // Remove the op code
        let mut params_instruction = self.get_instruction() / 100;
        let mut params = vec![0; num_params];
        for (i, param) in params.iter_mut().enumerate() {
            let position_mode = params_instruction % 10;
            match position_mode {
                0 => *param = self.position_parameter_address(i + 1),
                1 => *param = self.parameter_address(i + 1),
                2 => *param = self.relative_parameter_address(i + 1),
                _ => panic!("Unsupported position mode"),
            }
            params_instruction /= 10;
        }
        params
    }

    fn parameters_value(&self, num_params: usize) -> Vec<isize> {
        let params_loc = self.parameters_address(num_params);
        params_loc.iter().map(|loc| self.read(*loc)).collect()
    }

    fn halt(&mut self) {
        self.program_halted = true;
    }

    fn add(&mut self) {
        let loc = self.parameters_address(3);
        self.memory
            .insert(loc[2], self.read(loc[0]) + self.read(loc[1]));
        self.instruction_pointer += 4;
    }

    fn mul(&mut self) {
        let loc = self.parameters_address(3);
        self.write(loc[2], self.read(loc[0]) * self.read(loc[1]));
        self.instruction_pointer += 4;
    }

    fn input(&mut self) {
        let target = self.parameters_address(1)[0];
        let input = self.input.pop_front().expect("Need input!");
        self.write(target, input);
        self.instruction_pointer += 2;
    }

    fn output(&mut self) {
        let target = self.parameters_value(1)[0];
        self.output.push(target);
        // println!("intcode output: {}", target);
        self.instruction_pointer += 2;
    }

    fn jump_if_true(&mut self) {
        let params = self.parameters_value(2);
        if params[0] != 0 {
            self.instruction_pointer = params[1] as usize;
        } else {
            self.instruction_pointer += 3
        }
    }

    fn jump_if_false(&mut self) {
        let params = self.parameters_value(2);
        if params[0] == 0 {
            self.instruction_pointer = params[1] as usize;
        } else {
            self.instruction_pointer += 3
        }
    }

    fn less_than(&mut self) {
        let loc = self.parameters_address(3);
        self.write(loc[2], (self.read(loc[0]) < self.read(loc[1])) as isize);
        self.instruction_pointer += 4
    }

    fn equals(&mut self) {
        let loc = self.parameters_address(3);
        self.write(loc[2], (self.read(loc[0]) == self.read(loc[1])) as isize);
        self.instruction_pointer += 4;
    }

    fn set_relative_base(&mut self) {
        self.relative_base += self.parameters_value(1)[0];
        self.instruction_pointer += 2;
    }
}
