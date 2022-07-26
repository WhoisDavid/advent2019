use advent2019::{get_input, AdventResult};
use std::collections::{HashMap, HashSet};
use std::fmt;

const GRID_SIZE: usize = 5;
const GRID_ISIZE: isize = GRID_SIZE as isize;

fn main() -> AdventResult<()> {
    let bugs = get_input::<String>(24)?.first_column();
    let bugs: Vec<_> = bugs
        .iter()
        .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();
    solve_part1(&bugs);
    solve_part2(&bugs);
    Ok(())
}

fn solve_part1(input: &[Vec<bool>]) {
    println!("\nPart 1:\n");
    let mut eris = Eris::new(input);
    let res = eris.find_cycle();
    println!("Cycle: {}", res);
}

fn solve_part2(input: &[Vec<bool>]) {
    println!("\nPart 2:\n");
    let mut eris = RecursiveEris::new(input);
    let res = eris.bugs_after_n_minutes(200);
    println!("Bugs after 200 minutes: {}", res);
}

type Map<T> = [[T; GRID_SIZE]; GRID_SIZE];

#[derive(Default)]
struct Eris {
    map: Map<bool>,
}

impl Eris {
    const SURROUNDINGS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    fn new(map: &[Vec<bool>]) -> Self {
        let mut arr: Map<bool> = Default::default();
        for x in 0..GRID_SIZE {
            arr[x].clone_from_slice(&map[x])
        }
        Self { map: arr }
    }

    fn update_map(&mut self) {
        let neighbour_count = self.count_neighbours();
        self.update_map_from_count(neighbour_count);
    }

    fn count_neighbours(&self) -> Map<u8> {
        let mut neighbour_counts: Map<u8> = Default::default();
        // Count surrounding bugs
        for (x, row) in self.map.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                for &(i, j) in Self::SURROUNDINGS.iter() {
                    let i = x as isize + i;
                    let j = y as isize + j;
                    if (0..GRID_ISIZE).contains(&i) && (0..GRID_ISIZE).contains(&j) {
                        let i = i as usize;
                        let j = j as usize;
                        if self.map[i][j] {
                            neighbour_counts[x][y] += 1;
                        }
                    }
                }
            }
        }
        neighbour_counts
    }

    fn update_map_from_count(&mut self, neighbour_count: Map<u8>) {
        for (map_row, count_row) in self.map.iter_mut().zip(neighbour_count.iter()) {
            for (bug, count) in map_row.iter_mut().zip(count_row) {
                if *bug {
                    if *count != 1 {
                        *bug = false;
                    }
                } else if *count == 1 || *count == 2 {
                    *bug = true
                }
            }
        }
    }

    fn hash(&self) -> u32 {
        self.map
            .iter()
            .flatten()
            .fold((0, 1), |(mut d, mut base), &bug| {
                if bug {
                    d += base;
                }
                base *= 2;
                (d, base)
            })
            .0
    }

    fn find_cycle(&mut self) -> u32 {
        let mut history = HashSet::new();
        let mut hash = 0;
        while !history.contains(&hash) {
            history.insert(self.hash());
            self.update_map();
            hash = self.hash();
        }
        println!("First cycle:\n{}", self);
        hash
    }

    fn count_bugs(&self) -> usize {
        self.map.iter().flatten().filter(|c| **c).count()
    }
}

impl fmt::Display for Eris {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let v: Vec<_> = self
            .map
            .iter()
            .map(|v| {
                v.iter()
                    .map(|c| if *c { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect();

        writeln!(f, "{}", v.join("\n"))
    }
}

struct RecursiveEris {
    level: isize,
    min_level: isize,
    max_level: isize,
    recursive_map: HashMap<isize, Eris>,
}

impl RecursiveEris {
    const SURROUNDINGS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    fn new(map: &[Vec<bool>]) -> Self {
        let mut hm = HashMap::new();
        hm.insert(0, Eris::new(map));
        Self {
            level: 0,
            min_level: 0,
            max_level: 0,
            recursive_map: hm,
        }
    }

    fn current_map(&mut self) -> &mut Map<bool> {
        &mut self.recursive_map.entry(self.level).or_default().map
    }

    fn inner_map(&mut self) -> &mut Map<bool> {
        self.max_level = self.max_level.max(self.level + 1);
        &mut self.recursive_map.entry(self.level + 1).or_default().map
    }

    fn outer_map(&mut self) -> &mut Map<bool> {
        self.min_level = self.min_level.min(self.level - 1);
        &mut self.recursive_map.entry(self.level - 1).or_default().map
    }

    fn remove_empty_maps(&mut self) {
        // Remove empty maps
        let empty_levels: Vec<_> = self
            .recursive_map
            .iter()
            .filter(|(_, v)| v.hash() == 0)
            .map(|(k, _)| *k)
            .collect();
        for level in empty_levels {
            self.recursive_map.remove(&level);
        }
    }

    fn inner_edge_count(&mut self, dir: (isize, isize)) -> u8 {
        let mut count = 0u8;

        if dir.0 == 0 {
            for x in 0..GRID_SIZE {
                let y = if dir.1 == 1 { 0 } else { GRID_SIZE - 1 };
                if self.inner_map()[x][y] {
                    count += 1
                }
            }
        } else {
            for y in 0..GRID_SIZE {
                let x = if dir.0 == 1 { 0 } else { GRID_SIZE - 1 };
                if self.inner_map()[x][y] {
                    count += 1
                }
            }
        }

        count
    }

    fn adjacent_bug_count(&mut self, loc: (usize, usize)) -> u8 {
        let mut adjacent_bug = 0;
        for &dir in Self::SURROUNDINGS.iter() {
            let x = loc.0 as isize + dir.0;
            let y = loc.1 as isize + dir.1;
            adjacent_bug += match (x, y) {
                (2, 2) => self.inner_edge_count(dir),
                (-1, _) => self.outer_map()[1][2] as u8,
                (GRID_ISIZE, _) => self.outer_map()[3][2] as u8,
                (_, -1) => self.outer_map()[2][1] as u8,
                (_, GRID_ISIZE) => self.outer_map()[2][3] as u8,
                (i, j) => self.current_map()[i as usize][j as usize] as u8,
            };
        }
        adjacent_bug
    }

    fn count_neighbours(&mut self) -> Map<u8> {
        let mut neighbour_counts: Map<u8> = Default::default();
        for (x, row) in neighbour_counts.iter_mut().enumerate() {
            for (y, v) in row.iter_mut().enumerate() {
                if x == 2 && y == 2 {
                    continue;
                }
                *v = self.adjacent_bug_count((x, y));
            }
        }

        neighbour_counts
    }

    fn recursive_update(&mut self) {
        let mut level_counts = HashMap::new();
        for level in self.min_level - 1..=self.max_level + 1 {
            self.level = level;
            let neighbour_count = self.count_neighbours();
            level_counts.insert(self.level, neighbour_count);
        }

        for (level, count) in level_counts.into_iter() {
            if let Some(eris) = self.recursive_map.get_mut(&level) {
                eris.update_map_from_count(count);
            }
        }
    }

    fn count_bugs(&self) -> usize {
        self.recursive_map
            .values()
            .map(|eris| eris.count_bugs())
            .sum()
    }

    fn bugs_after_n_minutes(&mut self, n: usize) -> usize {
        for _ in 0..n {
            self.recursive_update();
        }
        self.remove_empty_maps();
        self.count_bugs()
    }
}

impl fmt::Display for RecursiveEris {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut keys: Vec<_> = self.recursive_map.keys().collect();
        keys.sort();

        keys.iter()
            .try_for_each(|level| writeln!(f, "Level {}:\n{}", level, self.recursive_map[level]))
    }
}

#[test]
fn test_day24_case1() {
    let bugs = &["....#", "#..#.", "#..##", "..#..", "#...."];
    let bugs: Vec<_> = bugs
        .iter()
        .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();
    let mut eris = Eris::new(&bugs);
    assert_eq!(eris.find_cycle(), 2_129_920);
}

#[test]
fn test_day24_case2() {
    let bugs = &["....#", "#..#.", "#.?##", "..#..", "#...."];
    let bugs: Vec<_> = bugs
        .iter()
        .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();
    let mut eris = RecursiveEris::new(&bugs);
    assert_eq!(eris.bugs_after_n_minutes(10), 99);
    println!("{}", eris);
}
