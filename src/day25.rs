use advent2019::intcode;
use advent2019::{get_input, AdventResult};
use std::io;

fn main() -> AdventResult<()> {
    let program = &get_input::<isize>(25)?.first_row();
    // let res = play(program)?;
    // println!("Commands played: {:?}", res);
    solve(program);
    Ok(())
}

fn solve(code: &[isize]) {
    let solution = [
        "east",
        "east",
        "take semiconductor",
        "north",
        "north",
        "take antenna",
        "south",
        "west",
        "take food ration",
        "west",
        "west",
        "take monolith",
        "east",
        "east",
        "east",
        "south",
        "east",
        "south",
        "south",
        "east",
        "east",
        "",
    ];

    let solution: Vec<_> = solution
        .join("\n")
        .chars()
        .map(|c| c as u8 as isize)
        .collect();
    let res = intcode::run_program(code, &solution);
    print_output(&res);
}

#[allow(dead_code)]
/// ### Map
/// The items are represented under the room name as `[item]` when usable and `(item)`when not.
/// ```text
///                                    Kitchen
///                               [weather machine]
///                                       |
///                                    Storage
///                                       |                                                     
///                                    Sick Bay ----------- Crew quarters
///                            [space law space brochure]       [jam]
///                                       |
///                                       |       Navigation -- Stables
///                                       |                    [antenna]
///                                       |                        |
///   Arcade -- Observatory -- Hot Chocolate Fountaine -- Gift Wrapping Center      Science Lab
/// [monolith]                       [food ration]           [planetoid]           (molten lava)
///                                                                |                     |
///    Corridor                                                    |                     |
/// (infinite loop)                                                |                     |
///       |                                                        |                     |
///  Hull Breach ------------- Engineering ------------- Warp Drive Maintenance ------ Hallway   
///                                                        [semiconductor]     (giant electromagnet)
///                                                                                      |
///                                                                                   Holodeck
///                                                                                 (escape pod)
///                                                                                      |
///                                                                                   Passage -- Security Checkpoint -- Pressure-Sensitive Floor
///                                                                                  (photons)                  
/// ```
///
/// ### Solution
/// There is a pressure sensitive door which requires to carry the right objects (to get to the right weight)
/// The final combination was:
/// - food ration
/// - antenna
/// - semiconductor
/// - monolith
///
/// Leading to the final output:
/// ```text
/// == Pressure-Sensitive Floor ==
/// Analyzing...
///
/// Doors here lead:
/// - west
///
/// A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit.
/// Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.
/// "Oh, hello! You should be able to get in by typing 268468864 on the keypad at the main airlock."
/// ```
/// Final set of commands:
/// ```
/// ["east", "east", "take semiconductor",
///  "north", "north", "take antenna",
///  "south", "west", "take food ration",
///  "west", "west", "take monolith",
///  "east", "east", "east", "south",
///  "east", "south", "south", "east", "east", ""]
/// ```
fn play(code: &[isize]) -> AdventResult<Vec<String>> {
    let mut intcode = intcode::IntCode::new(code);
    let mut input = Vec::new();

    let mut command_history = Vec::new();
    while !intcode.has_halted() {
        let res = intcode.run_till_input_needed(&input);
        print_output(&res);
        let mut command = String::new();
        // while !is_valid(&command) {
        command.clear();
        io::stdin().read_line(&mut command)?;
        // }
        input = command
            .chars()
            .map(|c| c as u8 as isize)
            .collect::<Vec<_>>();
        command_history.push(command);
    }
    Ok(command_history)
}

fn print_output(output: &[isize]) {
    println!(
        "{}",
        output.iter().map(|c| *c as u8 as char).collect::<String>()
    )
}
