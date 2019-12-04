use crate::{get_input, AdventResult};

pub fn solve(with_fuel: bool) -> AdventResult<u32> {
    let input = get_input::<u32>(1)?.first_column();

    let fuel = input.iter().map(|&m| fuel_reqs(m, with_fuel)).sum();

    println!("Required fuel: {}", fuel);
    Ok(fuel)
}

pub fn solve_part1() -> AdventResult<u32> {
    solve(false)
}

pub fn solve_part2() -> AdventResult<u32> {
    solve(true)
}

pub fn fuel_reqs(mass: u32, with_fuel: bool) -> u32 {
    let mut fuel = mass / 3 - 2;
    if with_fuel {
        let mut fuel_mass = fuel;
        while fuel_mass / 3 >= 2 {
            fuel_mass = fuel_mass / 3 - 2;
            fuel += fuel_mass
        }
    }
    fuel
}
