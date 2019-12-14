use advent2019::intcode::{run_program, IntCode};
use advent2019::{get_input, AdventResult};

fn main() -> AdventResult<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}

fn solve_part1() -> AdventResult<()> {
    let program = &get_input::<isize>(13)?.first_row();
    let screen_input = run_program(program, &[]);
    let block_tiles = count_block_tiles(&screen_input);
    println!("Block tiles on screen: {}", block_tiles);
    Ok(())
}

fn solve_part2() -> AdventResult<()> {
    let program = get_input::<isize>(13).expect("Program!").first_row();
    let score = play_game(program);
    println!("Final score: {}", score);
    Ok(())
}

fn count_block_tiles(input: &[isize]) -> usize {
    input
        // (x, y, tile_id)
        .chunks(3)
        // tile id == 2
        .filter(|s| s[2] == 2)
        .count()
}

fn play_game(mut program: Vec<isize>) -> isize {
    // Set address 0 to 2 to "play for free"
    program[0] = 2;

    let mut game = BrickBreaker::new(&program);

    game.run();
    game.score
}

struct BrickBreaker {
    intcode: IntCode,
    paddle: isize,
    ball: (isize, isize),
    score: isize,
}

impl BrickBreaker {
    fn new(program: &[isize]) -> Self {
        Self {
            intcode: IntCode::new(program),
            paddle: 0,
            ball: (0, 0),
            score: 0,
        }
    }

    fn run(&mut self) {
        let mut next_move_opt = None;
        while !self.intcode.has_halted() {
            let output = match next_move_opt {
                Some(next_move) => self.intcode.run_till_input(&[next_move]),
                None => self.intcode.run_till_input(&[]),
            };
            for tile in output.chunks(3) {
                let x = tile[0];
                let y = tile[1];
                let tile_id = tile[2];

                if x == -1 && y == 0 {
                    self.score = tile_id;
                    continue;
                }
                match tile_id {
                    0..=2 => (),
                    3 => self.paddle = x,
                    4 => self.ball = (x, y),
                    _ => panic!("Unexpected tile id!"),
                }
            }
            next_move_opt = Some((self.ball.0 - self.paddle).signum());
        }
    }
}
