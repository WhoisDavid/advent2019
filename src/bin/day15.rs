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
    let shortest_path_to_oxygen = shortest_path(intcode, 0, 0);
    println!("Shortest path to Oxygen: {}", shortest_path_to_oxygen);
    Ok(())
}

fn solve_part2(input: &[isize]) -> AdventResult<()> {
    let intcode = IntCode::new(input);
    let (shortest_path_to_oxygen, oxygen_intcode) = get_oxygen_state(intcode, 0, 0);
    println!("Shortest path to Oxygen: {}", shortest_path_to_oxygen);
    let longest_path_from_oxygen = longest_path(oxygen_intcode, 0, 0);
    println!("Longest path from Oxygen: {}", longest_path_from_oxygen);
    Ok(())
}

fn get_opposite_dir(dir: isize) -> isize {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("Unexpected dir"),
    }
}

fn shortest_path(intcode: IntCode, current_dist: usize, opposite_dir: isize) -> usize {
    let mut dist = usize::max_value();
    for dir in 1..=4 {
        // Never go back
        if dir == opposite_dir {
            continue;
        }
        let mut new_intcode = intcode.clone();
        let status = new_intcode.run_till_output(&[dir]);
        match status {
            0 => continue, // Wall
            1 => {
                dist = dist.min(shortest_path(
                    new_intcode,
                    current_dist + 1,
                    get_opposite_dir(dir),
                ));
            } // continue search
            2 => return current_dist + 1, // Found oxygen!,
            _ => panic!("Unexpected status!"),
        }
    }
    dist
}

fn get_oxygen_state(intcode: IntCode, distance: usize, opposite_dir: isize) -> (usize, IntCode) {
    let mut min_distance = usize::max_value();
    let mut oxygen_intcode = intcode.clone();
    for dir in 1..=4 {
        // Never go back
        if dir == opposite_dir {
            continue;
        }
        let mut new_intcode = intcode.clone();
        let status = new_intcode.run_till_output(&[dir]);
        match status {
            0 => continue, // Wall
            1 => {
                let (dist, new_intcode) =
                    get_oxygen_state(new_intcode, distance + 1, get_opposite_dir(dir));
                if dist < min_distance {
                    min_distance = dist;
                    oxygen_intcode = new_intcode;
                }
            } // continue search
            2 => return (distance + 1, new_intcode), // Found oxygen!,
            _ => panic!("Unexpected status!"),
        }
    }
    (min_distance, oxygen_intcode)
}

fn longest_path(oxygen_intcode: IntCode, current_dist: usize, opposite_dir: isize) -> usize {
    let mut dist = 0;
    for dir in 1..=4 {
        // Never go back
        if dir == opposite_dir {
            continue;
        }
        let mut new_intcode = oxygen_intcode.clone();
        let status = new_intcode.run_till_output(&[dir]);
        match status {
            0 => continue, // Wall
            1 => {
                dist = dist.max(longest_path(
                    new_intcode,
                    current_dist + 1,
                    get_opposite_dir(dir),
                ));
            } // continue search
            2 => panic!("Should not go back to start!"),
            _ => panic!("Unexpected status!"),
        }
    }
    dist.max(current_dist)
}
