pub fn run_program(code: &[isize], input: &[isize]) -> isize {
    let mut program = Program::new(code);
    program.run(input, true);
    program.output
}

pub fn run_program_iteration(code: &[isize], input: &[isize]) -> isize {
    let mut program = Program::new(code);
    program.run(input, false);
    program.output
}

pub struct Program {
    program: Vec<isize>,
    instruction_pointer: usize,
    input: Vec<isize>,
    input_cursor: usize,
    output: isize,
    program_halted: bool,
}

impl Program {
    pub fn new(program: &[isize]) -> Self {
        Self {
            program: program.to_vec(),
            instruction_pointer: 0,
            input: Vec::new(),
            input_cursor: 0,
            output: -1,
            program_halted: false,
        }
    }

    pub fn has_halted(&self) -> bool {
        self.program_halted
    }

    pub fn run(&mut self, input: &[isize], until_halt: bool) -> isize {
        // Set input
        self.input.append(&mut input.to_vec());
        self._run(until_halt)
    }

    pub fn run_iteration(&mut self, input: &[isize]) -> isize {
        // Set input
        self.input.append(&mut input.to_vec());
        self._run(false)
    }

    pub fn _run(&mut self, until_halt: bool) -> isize {
        if self.program_halted {
            return self.output;
        }
        let op = self.get_instruction() % 100;
        match op {
            1 => self.add(),
            2 => self.mul(),
            3 => self.input(),
            4 => {
                self.output();
                if !until_halt {
                    return self.output;
                }
            }
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equals(),
            99 => self.halt(),
            _ => panic!("Unsupported Op!"),
        };
        self._run(until_halt)
    }

    fn read_next_input(&mut self) -> isize {
        let input = self.input[self.input_cursor];
        self.input_cursor += 1;
        input
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

    fn halt(&mut self) {
        self.program_halted = true;
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
        self.program[target] = self.read_next_input();
        self.instruction_pointer += 2
    }

    fn output(&mut self) {
        let target = self.get_params(1)[0];
        self.output = target;
        // println!("Output: {}", target);
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
}
