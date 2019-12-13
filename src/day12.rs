use advent2019;
use advent2019::AdventResult;
use num::integer::lcm;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign};

fn main() -> AdventResult<()> {
    // Just used to download the file
    let _ = advent2019::get_input::<String>(12);
    solve_part1()?;
    solve_part2()?;
    Ok(())
}

fn get_input() -> System {
    System {
        moons: [
            Moon::new(&[-9, 10, -1]),
            Moon::new(&[-14, -8, 14]),
            Moon::new(&[1, 5, 6]),
            Moon::new(&[-19, 7, 8]),
        ],
    }
}

pub fn solve_part1() -> AdventResult<()> {
    let mut system = get_input();
    n_steps(&mut system, 1000);
    let total_energy = total_system_energy(&system);
    println!("Total energy: {}", total_energy);
    Ok(())
}

pub fn solve_part2() -> AdventResult<()> {
    let mut system = get_input();
    n_steps(&mut system, 1000);
    let cycle = cycle_length_lcm(&mut system);
    println!("Cycle length: {}", cycle);
    Ok(())
}

fn total_system_energy(system: &System) -> isize {
    system.moons.iter().map(|m| m.total_energy()).sum()
}

fn n_steps(system: &mut System, n: usize) {
    for _ in 0..n {
        next_step(system);
    }
}

fn next_step(system: &mut System) {
    for i in 1..system.moons.len() {
        let (moon1, moon2) = system.moons.split_at_mut(i);
        moon1[i - 1].gravity(moon2);
    }

    for moon in system.moons.iter_mut() {
        moon.update_position()
    }
}

fn _cycle_length_brute_force(system: &mut System) -> usize {
    let init_state = system.clone();
    let mut l = 1;
    next_step(system);
    while system != &init_state {
        next_step(system);
        l += 1;
        if l % 100_000_000 == 0 {
            println!("{}m steps", l / 1_000_000)
        }
    }

    l
}

fn cycle_length_lcm(system: &mut System) -> usize {
    // Find cycle per axis and output lcm(cycle_x, cycle_y, cycle_z)
    let (init_x, init_y, init_z) = system.get_axes();
    let (mut cycle_x, mut cycle_y, mut cycle_z) = (0, 0, 0);
    let (mut found_cycle_x, mut found_cycle_y, mut found_cycle_z) = (false, false, false);
    let mut cycle_len = 0;
    while !(found_cycle_x && found_cycle_y && found_cycle_z) {
        next_step(system);
        cycle_len += 1;
        let (x_axis, y_axis, z_axis) = system.get_axes();

        if !found_cycle_x && x_axis == init_x {
            cycle_x = cycle_len;
            found_cycle_x = true;
        }
        if !found_cycle_y && y_axis == init_y {
            cycle_y = cycle_len;
            found_cycle_y = true;
        }
        if !found_cycle_z && z_axis == init_z {
            cycle_z = cycle_len;
            found_cycle_z = true;
        }
    }
    lcm(lcm(cycle_x, cycle_y), cycle_z)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn abs_sum(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(PartialEq, Eq, Clone)]
struct System {
    moons: [Moon; 4],
}

type Axis = Vec<(isize, isize)>;

impl System {
    fn get_axes(&self) -> (Axis, Axis, Axis) {
        (
            self.moons.iter().map(|m| (m.pos.x, m.vel.x)).collect(),
            self.moons.iter().map(|m| (m.pos.y, m.vel.y)).collect(),
            self.moons.iter().map(|m| (m.pos.z, m.vel.z)).collect(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Moon {
    pos: Vector,
    vel: Vector,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "pos: ({}, {}, {}) vel: ({}, {}, {})",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z
        )
    }
}

macro_rules! axis_gravity {
    ($moon1:ident, $moon2:ident, $axis:ident) => {
        match $moon1.pos.$axis.cmp(&$moon2.pos.$axis) {
            Ordering::Equal => (),
            Ordering::Less => {
                $moon1.vel.$axis += 1;
                $moon2.vel.$axis -= 1
            }
            Ordering::Greater => {
                $moon1.vel.$axis -= 1;
                $moon2.vel.$axis += 1
            }
        }
    };
}

impl Moon {
    fn new(pos: &[isize; 3]) -> Self {
        Moon {
            pos: Vector {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            },
            vel: Vector { x: 0, y: 0, z: 0 },
        }
    }

    fn gravity(&mut self, others: &mut [Moon]) {
        for other in others {
            axis_gravity!(self, other, x);
            axis_gravity!(self, other, y);
            axis_gravity!(self, other, z);
        }
    }

    fn update_position(&mut self) {
        self.pos += &self.vel;
    }

    fn potential_energy(&self) -> isize {
        self.pos.abs_sum()
    }

    fn kinetic_energy(&self) -> isize {
        self.vel.abs_sum()
    }

    fn total_energy(&self) -> isize {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[test]
fn test_day12_case1() {
    let mut moons = System {
        moons: [
            Moon::new(&[-1, 0, 2]),
            Moon::new(&[2, -10, -7]),
            Moon::new(&[4, -8, 8]),
            Moon::new(&[3, 5, -1]),
        ],
    };
    n_steps(&mut moons, 10);
    assert_eq!(total_system_energy(&moons), 179);
}

#[test]
fn test_day12_case2() {
    let mut moons = System {
        moons: [
            Moon::new(&[-8, -10, 0]),
            Moon::new(&[5, 5, 10]),
            Moon::new(&[2, -7, 3]),
            Moon::new(&[9, -8, -3]),
        ],
    };
    n_steps(&mut moons, 100);
    assert_eq!(total_system_energy(&moons), 1940);
}

#[test]
fn test_day12_case1_part2() {
    let mut moons = System {
        moons: [
            Moon::new(&[-1, 0, 2]),
            Moon::new(&[2, -10, -7]),
            Moon::new(&[4, -8, 8]),
            Moon::new(&[3, 5, -1]),
        ],
    };
    assert_eq!(_cycle_length_brute_force(&mut moons), 2772);
    assert_eq!(cycle_length_lcm(&mut moons), 2772);
}

#[test]
fn test_day12_case2_part2() {
    let mut moons = System {
        moons: [
            Moon::new(&[-8, -10, 0]),
            Moon::new(&[5, 5, 10]),
            Moon::new(&[2, -7, 3]),
            Moon::new(&[9, -8, -3]),
        ],
    };
    assert_eq!(cycle_length_lcm(&mut moons), 4_686_774_924);
}
