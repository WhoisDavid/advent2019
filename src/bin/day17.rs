use advent2019::intcode;
use advent2019::{get_input, AdventResult};
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> AdventResult<()> {
    let program = &get_input::<isize>(17)?.first_row();
    solve_part1(program)?;
    solve_part2(program)?;
    Ok(())
}

fn solve_part1(input: &[isize]) -> AdventResult<()> {
    let grid = &parse_grid(input, true);
    let res = count_scaffold_intersections(grid);
    println!("Sum of scaffold intersections: {}", res);
    Ok(())
}

fn solve_part2(input: &[isize]) -> AdventResult<()> {
    let grid = &parse_grid(input, false);
    let mut s = Scaffold::new(grid);
    s.derive_path();
    let (main, a, b, c) = brute_force(&s.cmd).expect("Program");
    println!("Main program: {}", &main);
    println!("- A: {}", &a);
    println!("- B: {}", &b);
    println!("- C: {}", &c);

    let continuous_video_feed = "n\n".to_string();

    let program_input: Vec<_> = vec![main, a, b, c, continuous_video_feed]
        .join("\n")
        .chars()
        .map(|c| c as u8)
        .map(|c| c as isize)
        .collect();

    let mut input_override = input.to_vec();
    input_override[0] = 2;

    let output = intcode::run_program(&input_override, &program_input);
    println!("Dust collected: {:?}", output.last().unwrap());
    Ok(())
}

fn parse_grid(input: &[isize], print: bool) -> Vec<Vec<char>> {
    let output: Vec<isize> = intcode::run_program(input, &[]);
    let output_char: Vec<char> = output.iter().map(|d| (*d as u8) as char).collect();
    if print {
        println!("{}", output_char.iter().collect::<String>());
    }
    output_char
        .split(|c| *c == '\n')
        .filter(|row| !row.is_empty())
        .map(|r| r.to_vec())
        .collect()
}

fn count_scaffold_intersections(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut possible_row_interesections = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for j in 1..cols - 1 {
            if row[j - 1] == '#' && row[j] == '#' && row[j + 1] == '#' {
                possible_row_interesections.insert((i, j));
            }
        }
    }

    let mut possible_col_interesections = HashSet::new();
    for j in 0..cols {
        for i in 1..rows - 1 {
            if grid[i - 1][j] == '#' && grid[i][j] == '#' && grid[i + 1][j] == '#' {
                possible_col_interesections.insert((i, j));
            }
        }
    }
    let intersections = possible_row_interesections.intersection(&possible_col_interesections);
    intersections.map(|(i, j)| i * j).sum()
}

enum Turn {
    Left,
    Right,
}

impl Turn {
    fn to_cmd(&self) -> char {
        match self {
            Self::Left => 'L',
            Self::Right => 'R',
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn to_tuple(self) -> (isize, isize) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }
}

struct Scaffold {
    grid: Vec<Vec<char>>,
    pos: (usize, usize),
    dir: Dir,
    rows: usize,
    cols: usize,
    cmd: Vec<String>,
}

impl Scaffold {
    fn new(grid: &[Vec<char>]) -> Self {
        let start = Self::start_point(grid).expect("Start point!");
        Self {
            grid: grid.to_vec(),
            pos: start,
            dir: Dir::Up,
            rows: grid.len(),
            cols: grid[0].len(),
            cmd: Vec::new(),
        }
    }

    fn start_point(grid: &[Vec<char>]) -> Option<(usize, usize)> {
        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '^' {
                    return Some((j, i));
                }
            }
        }
        None
    }

    fn value(&self, pos: (usize, usize)) -> char {
        self.grid[pos.1][pos.0]
    }

    fn is_pos_valid(&self, pos: (isize, isize)) -> Option<(usize, usize)> {
        if 0 <= pos.0 && pos.0 < self.cols as isize && 0 <= pos.1 && pos.1 < self.rows as isize {
            Some((pos.0 as usize, pos.1 as usize))
        } else {
            None
        }
    }

    fn next_position(&self, dir: Dir) -> Option<(usize, usize)> {
        let dir_tup = dir.to_tuple();
        let new_pos = (
            self.pos.0 as isize + dir_tup.0,
            self.pos.1 as isize + dir_tup.1,
        );
        if let Some(new_pos) = self.is_pos_valid(new_pos) {
            if self.value(new_pos) == '#' {
                return Some((new_pos.0 as usize, new_pos.1 as usize));
            }
        }
        None
    }

    fn next_dir(&self) -> Option<Dir> {
        let possible_dirs = match self.dir {
            Dir::Left | Dir::Right => &[Dir::Up, Dir::Down],
            Dir::Up | Dir::Down => &[Dir::Left, Dir::Right],
        };

        for dir in possible_dirs {
            if self.next_position(*dir).is_some() {
                return Some(*dir);
            }
        }
        None
    }

    fn turn(&self, next_dir: Dir) -> Turn {
        match (self.dir, next_dir) {
            (Dir::Left, Dir::Down) => Turn::Left,
            (Dir::Left, Dir::Up) => Turn::Right,
            (Dir::Right, Dir::Down) => Turn::Right,
            (Dir::Right, Dir::Up) => Turn::Left,
            (Dir::Up, Dir::Left) => Turn::Left,
            (Dir::Up, Dir::Right) => Turn::Right,
            (Dir::Down, Dir::Left) => Turn::Right,
            (Dir::Down, Dir::Right) => Turn::Left,
            (_, _) => unreachable!(),
        }
    }

    fn derive_path(&mut self) {
        while let Some(d) = self.next_dir() {
            let turn = self.turn(d);
            self.cmd.push(turn.to_cmd().to_string());
            self.dir = d;
            let mut steps = 0u8;
            while let Some(pos) = self.next_position(self.dir) {
                steps += 1;
                self.pos = pos;
            }
            self.cmd.push(steps.to_string())
        }
    }
}

fn replace_slice<T>(buf: &mut Vec<T>, from: &[T], to: &[T])
where
    T: Clone + PartialEq,
{
    let mut i = 0;
    while i <= buf.len() - from.len() {
        if buf[i..].starts_with(from) {
            buf.splice(i..i + from.len(), to.iter().cloned());
            i += to.len()
        } else {
            i += 1
        }
    }
}

fn brute_force(input: &[String]) -> Option<(String, String, String, String)> {
    let funcs = ["A".to_string(), "B".to_string(), "C".to_string()];
    for a_len in (2..20).step_by(2) {
        let a = &input[..a_len];
        let mut input_a = input.to_vec();
        replace_slice(&mut input_a, a, &funcs[0..1]);
        for b_len in (2..20).step_by(2) {
            let mut input_b = input_a.to_vec();
            let b: Vec<String> = input_b
                .iter()
                .skip_while(|v| funcs.contains(v))
                .take_while(|v| !funcs.contains(v))
                .take(b_len)
                .cloned()
                .collect();
            if b.len() < b_len {
                break;
            }
            replace_slice(&mut input_b, &b, &funcs[1..2]);

            let c: Vec<String> = input_b
                .iter()
                .skip_while(|v| funcs.contains(v))
                .take_while(|v| !funcs.contains(v))
                .cloned()
                .collect();

            replace_slice(&mut input_b, &c, &funcs[2..3]);
            let left = input_b.iter().filter(|v| !funcs.contains(v)).count();
            if left == 0 {
                return Some((input_b.join(","), a.join(","), b.join(","), c.join(",")));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn pair_encoding(input: &[String], limit: usize) -> Vec<String> {
    let mut s = input.to_vec();
    let mut idx = 0;
    while s.len() > limit {
        idx += 1;

        let mut pairs: Vec<_> = s.chunks(2).collect();
        pairs.sort();
        let most_frequent_pair = pairs
            .iter()
            .group_by(|v| *v)
            .into_iter()
            .map(|(a, b)| (b.count(), *a))
            .max()
            .unwrap()
            .1;

        let mut pair = [String::new(), String::new()];
        pair.clone_from_slice(most_frequent_pair);
        replace_slice(&mut s, &pair, &[idx.to_string()]);
    }
    s
}
