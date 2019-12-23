use advent2019::intcode;
use advent2019::{get_input, AdventResult};

fn main() -> AdventResult<()> {
    let program = &get_input::<isize>(21)?.first_row();
    solve_part1(program);
    solve_part2(program);
    Ok(())
}

fn solve_part1(input: &[isize]) {
    // NOTE: a jump is 4 tiles long
    let springdroid_code = &[
        // (No tile ahead = Jump)
        "NOT A J", // J = !A
        // OR (No tile in 2)
        "NOT B T", // T = !B
        "OR T J",  // J = J || T = !A || !B
        // OR (No tile in 3)
        "NOT C T", // T = !C
        "OR T J",  // J = J || T = !A || !B || !C
        // AND (tile in 4 = landing)
        "AND D J", // J = D && J = D && (!A || !B || !C)
        "WALK",    // run command
        "",
    ];

    run_springdroid(input, springdroid_code)
}

fn solve_part2(input: &[isize]) {
    // NOTE: a jump is 4 tiles long
    let springdroid_code = &[
        // (No tile ahead = Jump)
        "NOT A J", // J = !A
        // OR (No tile in 2)
        "NOT B T", // T = !B
        "OR T J",  // J = J || T = !A || !B
        // OR (No tile in 3)
        "NOT C T", // T = !C
        "OR T J",  // J = J || T = !A || !B || !C
        // AND (tile in 4 = landing)
        "AND D J", // J = D && J = D && (!A || !B || !C) <= Same as Part 1
        // AND (tile in 5 [= tile after landing] OR tile in 8 [= second jump landing])
        "NOT E T", // T = !E
        "NOT T T", // T = E
        "OR H T",  // T = E || H
        "AND T J", // J = J && T = part1 && (E || H)
        "RUN",     // run command
        "",
    ];

    run_springdroid(input, springdroid_code)
}

fn run_springdroid(input: &[isize], springdroid_code: &[&str]) {
    let springdroid_intcode_input: Vec<isize> = springdroid_code
        .join("\n")
        .chars()
        .map(|c| c as u8 as isize)
        .collect();

    let res = intcode::run_program(input, &springdroid_intcode_input);

    println!(
        "Springdroid output:\n{}",
        res.iter()
            .flat_map(|c| if 0 <= *c && *c < 256 {
                vec![*c as u8 as char]
            } else {
                c.to_string().chars().collect::<Vec<_>>()
            })
            .collect::<String>()
    );
}
