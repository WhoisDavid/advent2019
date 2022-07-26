use advent2019::{get_input_with_params, AdventResult};

fn main() -> AdventResult<()> {
    let input = get_input_with_params::<u64>(4, false, '-')?.first_row();
    let lowerbound = input[0];
    let upperbound = input[1];

    solve_part1(lowerbound, upperbound)?;
    solve_part2(lowerbound, upperbound)?;
    Ok(())
}

pub fn solve_part1(lowerbound: u64, upperbound: u64) -> AdventResult<usize> {
    let res = (lowerbound..=upperbound)
        .filter(|&n| validate_rules(n, false))
        .count();
    println!("Result: {}", res);
    Ok(res)
}

pub fn solve_part2(lowerbound: u64, upperbound: u64) -> AdventResult<usize> {
    let res = (lowerbound..=upperbound)
        .filter(|&n| validate_rules(n, true))
        .count();
    println!("Result: {}", res);
    Ok(res)
}

pub fn number_to_vec(n: u64) -> Vec<u64> {
    let mut digits = Vec::with_capacity(10);
    let mut n = n;

    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

pub fn validate_rules(n: u64, part_2: bool) -> bool {
    /*
    - It is a six-digit number.
    - The value is within the range given in your puzzle input.
    - Two adjacent digits are the same (like 22 in 122345).
    - Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
     */
    let digits = number_to_vec(n);

    let right_length = digits.len() == 6;
    if !right_length {
        return false;
    }

    let mut same_consecutive_digits = false;
    let mut same_consecutive_digits_part2 = false;
    let mut same_digits_count = 0;
    let mut decreasing_digits = true;
    let mut prev_digit = 0;
    for &digit in digits.iter() {
        if digit == prev_digit {
            same_digits_count += 1;
            same_consecutive_digits = true
        } else {
            if same_digits_count == 1 {
                same_consecutive_digits_part2 = true;
            }
            same_digits_count = 0;
        }
        decreasing_digits &= digit >= prev_digit;
        prev_digit = digit;
    }

    if same_digits_count == 1 {
        same_consecutive_digits_part2 = true;
    }

    let is_valid = same_consecutive_digits && decreasing_digits;
    if part_2 {
        is_valid && same_consecutive_digits_part2
    } else {
        is_valid
    }
}

#[test]
fn test_cases_part1() {
    assert!(validate_rules(111_111, false));
    assert!(!validate_rules(223_450, false));
    assert!(!validate_rules(123_789, false));
}

#[test]
fn test_cases_part2() {
    assert!(validate_rules(112_233, true));
    assert!(!validate_rules(123_444, true));
    assert!(validate_rules(111_122, true));
}
