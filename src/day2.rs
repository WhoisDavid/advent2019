use crate::{get_input, AdventError, AdventResult};

pub fn solve_part1() -> AdventResult<usize> {
    let mut input = get_input::<usize>(2)?.first_row();

    // "1202 program alarm" state
    // replace position 1 with the value 12 and replace position 2 with the value 2
    input[1] = 12;
    input[2] = 2;

    let res = run_program(&mut input)?[0];
    println!("Result: {}", res);
    Ok(res)
}

pub fn solve_part2() -> AdventResult<usize> {
    let input = get_input::<usize>(2)?.first_row();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_mut = input.to_vec();
            input_mut[1] = noun;
            input_mut[2] = verb;
            let d = run_program(&mut input_mut)?[0];
            if d == 19_690_720 {
                println!("Found pair (noun={}, verb={}) !", noun, verb);
                let res = 100 * noun + verb;
                println!("Result: {}", res);
                return Ok(res);
            }
        }
    }

    Err(AdventError::InvalidValue)
}

pub fn run_program(input: &mut Vec<usize>) -> AdventResult<&Vec<usize>> {
    let mut idx = 0;
    while input[idx] != 99 {
        let op = input[idx];
        let v1 = input[input[idx + 1]];
        let v2 = input[input[idx + 2]];
        let dst = input[idx + 3];

        input[dst] = match op {
            1 => v1 + v2,
            2 => v1 * v2,
            _ => return Err(AdventError::InvalidValue),
        };
        idx += 4;
    }

    Ok(input)
}

#[test]
fn test_program() {
    assert_eq!(
        &vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        run_program(&mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).unwrap()
    );
}
