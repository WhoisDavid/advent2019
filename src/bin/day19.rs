use advent2019::intcode::get_program_last_output;
use advent2019::{get_input, AdventResult};
use std::collections::HashMap;
use SearchResult::*;

fn main() -> AdventResult<()> {
    let program = &get_input::<isize>(19)?.first_row();
    solve_part1(program)?;
    solve_part2(program)?;
    Ok(())
}

fn solve_part1(input: &[isize]) -> AdventResult<()> {
    let mut res = HashMap::new();
    for i in 0..50 {
        for j in 0..50 {
            let pull = get_program_last_output(input, &[i, j]);
            res.insert((i, j), pull);
            // Prints the points
            // if pull > 0 {
            // println!("{}\t{}\t{}", i, j, pull);
            // }
        }
    }

    let points_affected = res.iter().filter(|(_, pull)| **pull > 0).count();
    println!("Points affected: {}", points_affected);

    Ok(())
}

fn solve_part2(input: &[isize]) -> AdventResult<()> {
    let bs = &mut BeamSearch::new(input);

    let (x, y) = bs.search_square(100);
    println!("Best: ({} {}) => {:?}", x, y, bs.search(x, y));
    println!("Result: {:?}", (x - 99) * 10000 + y);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum SearchResult {
    OutsideOfBeam,
    NotAnEdge,
    TooSmall,
    TooBig,
    Valid,
}

struct BeamSearch<'a> {
    code: &'a [isize],
    results: HashMap<(isize, isize), isize>,
}

impl<'a> BeamSearch<'a> {
    fn new(code: &'a [isize]) -> Self {
        Self {
            code,
            results: HashMap::new(),
        }
    }

    fn run(&mut self, x: isize, y: isize) -> isize {
        if let Some(cache) = self.results.get(&(x, y)) {
            return *cache;
        }
        let res = get_program_last_output(self.code, &[x, y]);
        self.results.insert((x, y), res);
        res
    }

    fn search(&mut self, x: isize, y: isize) -> SearchResult {
        let square = 100 - 1;

        let output = self.run(x, y);
        if output == 0 {
            return OutsideOfBeam;
        }
        let is_edge = self.run(x, y - 1) == 0;
        if !is_edge {
            return NotAnEdge;
        }

        let fits_square = self.run(x - square, y + square) > 0;

        if !fits_square {
            return SearchResult::TooSmall;
        }

        let too_big = self.run(x - square, y + square + 1) > 0;
        if too_big {
            return TooBig;
        }

        return Valid;
    }

    // Binary search the y-axis to find the lower edge of the beam given x
    fn search_edge(&mut self, x: isize) -> (isize, SearchResult) {
        let mut lower = x;
        let mut upper = 3 * x; // outside the beam

        // Make sure we start with lower outside and upper in the beam
        while self.search(x, lower) != OutsideOfBeam {
            lower /= 2;
        }
        while self.search(x, upper) == OutsideOfBeam {
            upper -= x / 10;
        }

        let mut mid = lower + (upper - lower) / 2;
        while upper - lower > 1 {
            match self.search(x, mid) {
                OutsideOfBeam => {
                    lower = mid;
                }
                NotAnEdge => upper = mid,
                search_result => return (mid, search_result),
            }
            mid = lower + (upper - lower) / 2;
        }

        (mid, self.search(x, mid))
    }

    // Search the lowest edge corner using a "double" binary search
    // One for the x-axis looking to fit the square
    // One for the y-axis to look for the bottom edge given x
    fn search_square(&mut self, lower: isize) -> (isize, isize) {
        let mut lower = lower;
        let mut upper = 2 * lower;
        // Make sure we're wide enough
        while self.search_edge(lower).1 != TooSmall {
            lower /= 2;
        }

        while self.search_edge(upper).1 != TooBig {
            upper *= 2;
        }

        let mut mid = lower + (upper - lower) / 2;
        while upper - lower > 1 {
            match self.search_edge(mid).1 {
                TooSmall => {
                    lower = mid;
                }
                TooBig => upper = mid,
                Valid => break,
                _ => panic!("Unexpected result"),
            }
            mid = lower + (upper - lower) / 2;
        }

        // Check for another minima around the found x
        // Since there can be local minima because of pixellization
        let x_res = mid;
        let mut x = x_res;
        let mut y = self.search_edge(x_res).0;
        for dx in 1..=20 {
            match self.search_edge(x_res - dx) {
                (new_y, Valid) => {
                    x = x_res - dx;
                    y = new_y;
                }
                _ => (),
            };
        }
        (x, y)
    }
}
