// use advent2019::day1;
use advent2019::*;

macro_rules! run_day {
    ($module:ident, $func_name:ident) => {
        println!("\n{}::{}", stringify!($module), stringify!($func_name));
        if let Err(e) = $module::$func_name() {
            println!("ERROR: {}", e);
        }
    };
}

fn main() {
    // println!("{:?}", get_input::<String>(3)?[0];

    run_day!(day3, solve_part1);
    run_day!(day3, solve_part2);
    // run_day!(day2, solve_part2);

}