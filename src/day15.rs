use advent2019::intcode::IntCode;
use advent2019::{get_input, AdventResult};

fn main() -> AdventResult<()> {
    let program = &get_input::<isize>(15)?.first_row();
    solve_part1(program)?;
    solve_part2(program)?;
    Ok(())
}

fn solve_part1(input: &[isize]) -> AdventResult<()> {
    let intcode = IntCode::new(input);
    let starting_loc = &Location {
        x: 0,
        y: 0,
        dist: 0,
        back_dir: 0,
    };
    let shortest_path_to_oxygen = shortest_path(intcode, starting_loc);
    println!("Shortest path to Oxygen: {}", shortest_path_to_oxygen);
    Ok(())
}

fn solve_part2(input: &[isize]) -> AdventResult<()> {
    let intcode = IntCode::new(input);
    let starting_loc = &Location {
        x: 0,
        y: 0,
        dist: 0,
        back_dir: 0,
    };
    let (shortest_path_to_oxygen, oxygen_intcode) = get_oxygen_intcode(intcode, starting_loc);
    println!("Shortest path to Oxygen: {}", shortest_path_to_oxygen);
    let longest_path_from_oxygen = longest_path(oxygen_intcode, starting_loc);
    println!("Longest path from Oxygen: {}", longest_path_from_oxygen);
    Ok(())
}

struct Location {
    x: isize,
    y: isize,
    dist: usize,
    back_dir: isize,
}

impl Location {
    fn new_move(&self, dir: isize) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        let back_dir: isize;
        match dir {
            1 => {
                y -= 1;
                back_dir = 2
            }
            2 => {
                y += 1;
                back_dir = 1
            }
            3 => {
                x -= 1;
                back_dir = 4
            }
            4 => {
                x += 1;
                back_dir = 3
            }
            _ => panic!("unexpected dir!"),
        }

        Self {
            x,
            y,
            dist: self.dist + 1,
            back_dir,
        }
    }
}

fn shortest_path(intcode: IntCode, loc: &Location) -> usize {
    let mut dist = usize::max_value();
    for dir in 1..=4 {
        // Never go back
        if dir == loc.back_dir {
            continue;
        }
        let mut new_intcode = intcode.clone();
        let status = new_intcode.run_till_output(&[dir]);
        match status {
            0 => continue, // Wall
            1 => {
                let loc = &loc.new_move(dir);
                dist = dist.min(shortest_path(new_intcode, loc));
            } // continue search
            2 => return loc.dist + 1, // Found oxygen!,
            _ => panic!("Unexpected status!"),
        }
    }
    dist
}

fn get_oxygen_intcode(intcode: IntCode, loc: &Location) -> (usize, IntCode) {
    let mut dist = usize::max_value();
    let mut oxygen_intcode = intcode.clone();
    for dir in 1..=4 {
        // Never go back
        if dir == loc.back_dir {
            continue;
        }
        let mut new_intcode = intcode.clone();
        let status = new_intcode.run_till_output(&[dir]);
        match status {
            0 => continue, // Wall
            1 => {
                let loc = &loc.new_move(dir);
                let (new_dist, new_intcode) = get_oxygen_intcode(new_intcode, loc);
                if new_dist < dist {
                    dist = new_dist;
                    oxygen_intcode = new_intcode;
                }
            } // continue search
            2 => return (loc.dist + 1, new_intcode), // Found oxygen!,
            _ => panic!("Unexpected status!"),
        }
    }
    (dist, oxygen_intcode)
}

fn longest_path(oxygen_intcode: IntCode, loc: &Location) -> usize {
    let mut dist = 0;
    for dir in 1..=4 {
        // Never go back
        if dir == loc.back_dir {
            continue;
        }
        let mut new_intcode = oxygen_intcode.clone();
        let status = new_intcode.run_till_output(&[dir]);
        match status {
            0 => continue, // Wall
            1 => {
                let loc = &loc.new_move(dir);
                dist = dist.max(longest_path(new_intcode, loc));
            } // continue search
            2 => panic!("Should not go back to start!"),
            _ => panic!("Unexpected status!"),
        }
    }
    dist.max(loc.dist)
}
