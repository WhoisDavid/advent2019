use advent2019::{get_input, AdventResult};

const PATTERN: &[isize] = &[0, 1, 0, -1];

fn main() -> AdventResult<()> {
    let input = get_input::<String>(16)?.first_element();
    let input = &string_to_vec(input);
    solve_part1(input)?;
    solve_part2(input)?;
    Ok(())
}

fn solve_part1(input: &[isize]) -> AdventResult<()> {
    let res = fft(input, 100);
    println!("FFT output: {}", vec_to_string(&res[..8]));
    Ok(())
}

fn solve_part2(input: &[isize]) -> AdventResult<()> {
    let res = fft_real(input, 100);
    println!("FFT real signal output: {}", vec_to_string(&res[..8]));
    Ok(())
}

fn string_to_vec<T: AsRef<str>>(input: T) -> Vec<isize> {
    input
        .as_ref()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

fn vec_to_string(input: &[isize]) -> String {
    input
        .iter()
        .map(|c| std::char::from_digit(*c as u32, 10).unwrap())
        .collect()
}

#[derive(Clone)]
struct Pattern {
    idx: usize,
    count: usize,
    position: usize,
}

impl Iterator for Pattern {
    type Item = isize;
    fn next(&mut self) -> Option<isize> {
        self.count += 1;
        if self.count > self.position {
            self.idx = (self.idx + 1) % PATTERN.len();
            self.count = 1;
        }

        Some(PATTERN[self.idx])
    }
}

fn pattern(position: usize) -> impl Iterator<Item = isize> {
    Pattern {
        idx: 0,
        count: 0,
        position,
    }
    .cycle()
    .skip(1)
}

fn fft(input: &[isize], phases: usize) -> Vec<isize> {
    let mut res = input.to_vec();
    for _ in 0..phases {
        for position in 1..=res.len() {
            let pat = pattern(position);
            let output: isize = res
                .iter()
                .zip(pat)
                .skip(position - 1)
                .map(|(v, p)| v * p)
                .sum();
            res[position - 1] = output.abs() % 10;
        }
    }
    res
}

fn fft_real(input: &[isize], phases: usize) -> Vec<isize> {
    let offset = input[..7].iter().fold(0, |acc, d| acc * 10 + d) as usize;
    let real_signal_len = 10_000 * input.len();
    let mut res: Vec<isize> = input
        .iter()
        .cycle()
        .take(real_signal_len)
        .skip(offset)
        .copied()
        .collect();

    res.reverse();
    for _ in 0..phases {
        let mut prev = 0;
        for r in res.iter_mut() {
            *r = (prev + *r) % 10;
            prev = *r;
        }
    }
    res.reverse();
    res[..8].to_vec()
}

#[allow(dead_code)]
fn fft_real_partial_sum(input: &[isize], phases: usize, offset: usize) -> Vec<isize> {
    let real_signal_len = 10_000 * input.len();
    let mut res: Vec<isize> = input
        .iter()
        .cycle()
        .take(real_signal_len)
        .skip(offset)
        .copied()
        .collect();

    for _ in 0..phases {
        let mut partial_sums = Vec::with_capacity(res.len() + 1);
        partial_sums.push(0);
        let mut prev = 0;
        for r in res.iter() {
            partial_sums.push(*r  + prev);
            prev += *r;
        }

        let len = res.len();
        for (i, r) in res.iter_mut().enumerate() {
            let mut output = 0;
            for idx in (i..=len).step_by(4 * (offset+i)) {
                output += partial_sums[(offset + idx).min(len)] - partial_sums[idx];
            }

            for idx in (3*(offset+i)..=len).step_by(4 * (offset+i)) {
                output -= partial_sums[(offset + idx).min(len)] - partial_sums[idx];
            }
            *r = output.abs() % 10;
        }
    }
    res[..8].to_vec()
}

#[allow(dead_code)]
fn test_part_1(input: &str, phases: usize, expected_output: Vec<isize>) {
    let input = string_to_vec(input);
    let output = fft(&input, phases);
    assert_eq!(output[..8].to_vec(), expected_output)
}

#[allow(dead_code)]
fn test_part_2(input: &str, phases: usize, expected_output: Vec<isize>) {
    let input = string_to_vec(input);
    let output = fft_real(&input, phases);
    assert_eq!(output, expected_output)
}
#[allow(dead_code)]
fn test_part_2_partial_sum(input: &str, phases: usize, expected_output: Vec<isize>) {
    let input = string_to_vec(input);
    let offset = input[..7].iter().fold(0, |acc, d| acc * 10 + d) as usize;
    let output = fft_real_partial_sum(&input, phases, offset);
    assert_eq!(output, expected_output)
}

#[test]
fn test_day16_case1() {
    test_part_1("12345678", 4, vec![0, 1, 0, 2, 9, 4, 9, 8])
}

#[test]
fn test_day16_case2() {
    test_part_1(
        "80871224585914546619083218645595",
        100,
        vec![2, 4, 1, 7, 6, 1, 7, 6],
    )
}

#[test]
fn test_day16_case3() {
    test_part_1(
        "19617804207202209144916044189917",
        100,
        vec![7, 3, 7, 4, 5, 4, 1, 8],
    )
}

#[test]
fn test_day16_case4() {
    test_part_1(
        "69317163492948606335995924319873",
        100,
        vec![5, 2, 4, 3, 2, 1, 3, 3],
    )
}

#[test]
fn test_day16_part2_case1() {
    test_part_2(
        "03036732577212944063491565474664",
        100,
        vec![8, 4, 4, 6, 2, 0, 2, 6],
    )
}

#[test]
fn test_day16_part2_case2() {
    test_part_2(
        "02935109699940807407585447034323",
        100,
        vec![7, 8, 7, 2, 5, 2, 7, 0],
    )
}

#[test]
fn test_day16_part2_case3() {
    test_part_2(
        "03081770884921959731165446850517",
        100,
        vec![5, 3, 5, 5, 3, 7, 3, 1],
    )
}
