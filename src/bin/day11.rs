use advent2019::intcode::IntCode;
use advent2019::{get_input, AdventResult};
use std::collections::HashMap;

fn main() -> AdventResult<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}

pub fn solve_part1() -> AdventResult<usize> {
    let program = &get_input::<isize>(11)?.first_row();
    let res = tiles_painted(program);
    println!("Tiles painted: {:?}", res);
    Ok(res)
}

pub fn solve_part2() -> AdventResult<()> {
    let program = &get_input::<isize>(11).expect("Program!").first_row();
    print_painting(program);
    Ok(())
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct PaintRobot {
    intcode: IntCode,
    hull: HashMap<(isize, isize), isize>,
    position: (isize, isize),
    direction: isize,
}

impl<'a> PaintRobot {
    const DIRS: &'a [Direction] = &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    fn new(program: &[isize]) -> Self {
        PaintRobot {
            intcode: IntCode::new(program),
            hull: HashMap::new(),
            position: (0, 0),
            direction: 0,
        }
    }

    fn paint_hull(&mut self, color: isize) {
        match color {
            0 => {
                self.hull.insert(self.position, 0);
            }
            1 => {
                self.hull.insert(self.position, 1);
            }
            _ => panic!("Unexpected color!"),
        }
    }

    fn run(&mut self, init_color: isize) {
        let mut color = init_color;
        while !self.intcode.has_halted() {
            color = self.intcode.run_till_output(&[color]);
            self.paint_hull(color);
            let dir = self.intcode.run_till_output(&[]);
            self.turn_and_move(dir);
            color = *self.hull.get(&self.position).unwrap_or(&0);
        }
    }

    fn turn_and_move(&mut self, input: isize) {
        match input {
            0 => self.direction = (self.direction - 1).rem_euclid(4),
            1 => self.direction = (self.direction + 1).rem_euclid(4),
            _ => panic!("Unexpected direction!"),
        };

        match Self::DIRS[self.direction as usize] {
            Direction::Up => self.position.1 -= 1,
            Direction::Right => self.position.0 += 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
        }
    }
}

pub fn tiles_painted(program: &[isize]) -> usize {
    let mut robot = PaintRobot::new(program);
    robot.run(0);
    robot.hull.len()
}

pub fn print_painting(program: &[isize]) {
    let mut robot = PaintRobot::new(program);
    robot.run(1);

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for &(x, y) in robot.hull.keys() {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let mut hull = vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(1) = robot.hull.get(&(x, y)) {
                hull[(y + min_y) as usize][(x + min_x) as usize] = '#';
            }
        }
    }

    hull.iter()
        .map(|c| c.iter().collect::<String>())
        .for_each(|s| println!("{}", s));
}
