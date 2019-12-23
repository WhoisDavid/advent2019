use advent2019::{get_input, AdventError, AdventResult};

fn main() -> AdventResult<()> {
    let program = &get_input::<String>(22)?.first_column();
    solve_part1(program)?;
    solve_part2(program)?;
    Ok(())
}

fn solve_part1(input: &[String]) -> AdventResult<()> {
    let shuffles = input
        .iter()
        .map(parse_shuffle)
        .collect::<AdventResult<Vec<_>>>()?;
    let res = shuffle_index(2019, 10007, &shuffles);
    println!("New index for card 2019: {}", res);
    Ok(())
}

fn solve_part2(input: &[String]) -> AdventResult<()> {
    // The deck size and number of shuffles are both prime
    let deck_size: i128 = 119_315_717_514_047;
    let number_of_shuffles: i128 = 101_741_582_076_661;
    let shuffles = input
        .iter()
        .map(parse_shuffle)
        .collect::<AdventResult<Vec<_>>>()?;
    let res = calculate_part2(2020, deck_size, number_of_shuffles, &shuffles);
    println!("Compound: {:?}", res);
    Ok(())
}

#[derive(Clone, Copy)]
enum Shuffle {
    DealIntoNewStack,
    CutFromTop(usize),
    CutFromBottom(usize),
    DealWithIncrement(usize),
}

fn parse_shuffle<T: AsRef<str>>(input: T) -> AdventResult<Shuffle> {
    let input = input.as_ref();

    if input.contains("stack") {
        return Ok(Shuffle::DealIntoNewStack);
    }

    if input.contains("cut") {
        let n: isize = input.split_whitespace().last().expect("Number!").parse()?;
        if n > 0 {
            return Ok(Shuffle::CutFromTop(n as usize));
        } else {
            return Ok(Shuffle::CutFromBottom(-n as usize));
        }
    }

    if input.contains("increment") {
        let n: usize = input.split_whitespace().last().expect("Number!").parse()?;
        return Ok(Shuffle::DealWithIncrement(n as usize));
    }

    Err(AdventError::InvalidValue)
}

fn shuffle_index(init_index: usize, deck_size: usize, shuffle: &[Shuffle]) -> usize {
    shuffle
        .iter()
        .fold(init_index, |index, technique| match *technique {
            Shuffle::DealIntoNewStack => deck_size - (index + 1),
            Shuffle::CutFromTop(n) => (deck_size + index - n) % deck_size,
            Shuffle::CutFromBottom(n) => (index + n) % deck_size,
            Shuffle::DealWithIncrement(n) => (index * n) % deck_size,
        })
}

fn technique_to_ab(technique: Shuffle, deck_size: i128) -> (i128, i128) {
    match technique {
        Shuffle::DealIntoNewStack => (-1, deck_size - 1),
        Shuffle::CutFromTop(n) => (1, deck_size - n as i128),
        Shuffle::CutFromBottom(n) => (1, n as i128),
        Shuffle::DealWithIncrement(n) => (n as i128, 0),
    }
}

/// Modulo deck_size
/// deal with increment n = (index * n) % deck_size =  n * idx + 0
/// deal into new stack   = deck_size - (index + 1) = -1 * idx + (deck_size - 1)
/// cut n                 = (deck_size + index - n) =  1 * idx + (deck_size - n)
/// cut -n                = (index + n)             =  1 * idx + (n)
fn compound_shuffle(deck_size: i128, shuffle: &[Shuffle]) -> (i128, i128) {
    // Represent the shuffle as ax + b
    let mut compound_a: i128 = 1;
    let mut compound_b: i128 = 0;
    for &technique in shuffle {
        let (a, b) = technique_to_ab(technique, deck_size);
        compound_a *= a;
        compound_b = a * compound_b + b;

        compound_a = compound_a.rem_euclid(deck_size);
        compound_b = compound_b.rem_euclid(deck_size);
    }
    (compound_a, compound_b)
}

/// Calculates a^b mod m using exponentiation by squaring
fn mod_exp(a: i128, b: i128, m: i128) -> i128 {
    if m == 1 {
        return 0;
    }
    let mut res = 1;
    let mut a = a % m;
    let mut b = b;
    while b > 0 {
        if b & 1 > 0 {
            res = res * a % m;
        }
        b >>= 1;
        a = a * a % m
    }
    res
}

/// Inverse prime modulo of a mod d
/// d is prime so a^-1 mod d = a^(\phi{d}-1) = a^(d-2)
fn mod_inv_prime(a: i128, d: i128) -> i128 {
    mod_exp(a, d - 2, d)
}

/// Lots of math happening here:
/// x: index, d: deck size, n: number of shuffle
/// We represent the shuffle as a linear transformation of the initial index `x`: (ax + b) mod d
/// Applying the shuffle n times is just a compound of this:
/// ```
/// index = a*(a*(a*...(ax+b)+b)+b)+b = a^n*x + b(1+a+a^2+...+a^(n-1))
/// index = a^n*x + b ( a^n - 1 ) * mod_inv(a - 1)`  mod d
///       = a^n ( x + b * mod_inv(a - 1, d) ) - b * mod_inv(a - 1, d)
///       = A * (x + B) - B mod d
/// where `A = a^n mod d` and `B = b * mod_inv(a - 1, d)  mod d`
/// ```
/// Looking for the final value at position `index`
/// ```
/// index = A * (x + B) - B mod d <=> x = (index + B) * mod_inv(A, d) - B
/// mod_inv(A, d) = mod_inv_prime(a^n) = mod_inv_prime(a)^n
/// ```
/// Thus:
/// ```
/// x = (index + B) * mod_inv_prime(a)^n - b
/// ```
fn calculate_part2(index: i128, d: i128, n: i128, shuffle: &[Shuffle]) -> i128 {
    let (a, b) = compound_shuffle(d, shuffle);
    println!("Compound shuffle ax + b: (a, b) = ({}, {})", a, b);
    let a_inv = mod_exp(a - 1, d - 2, d);
    let b = (b * a_inv) % d;
    ((index + b) * mod_exp(mod_inv_prime(a, d), n, d) - b) % d
}

#[cfg(test)]
fn test_shuffle<T: AsRef<str>>(shuffle: &[T], input: &[usize], expected: &[usize]) {
    let shuffles = shuffle
        .iter()
        .map(|s| parse_shuffle(s.as_ref()))
        .collect::<AdventResult<Vec<_>>>()
        .expect("Shuffle instructions");

    let mut res = vec![0; input.len()];
    for &idx in input.iter() {
        let new_idx = shuffle_index(idx, input.len(), &shuffles);
        res[new_idx] = idx;
    }
    assert_eq!(res, expected);
}

#[test]
fn test_day22_stack() {
    let shuffle = &["deal into new stack"];

    let input: &[usize] = &(0..10).collect::<Vec<_>>();
    let expected = &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    test_shuffle(shuffle, input, expected);
}

#[test]
fn test_day22_cut_top() {
    let shuffle = &["cut 3"];

    let input: &[usize] = &(0..10).collect::<Vec<_>>();
    let expected = &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2];

    test_shuffle(shuffle, input, expected);
}

#[test]
fn test_day22_cut_bottom() {
    let shuffle = &["cut -4"];

    let input: &[usize] = &(0..10).collect::<Vec<_>>();
    let expected = &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5];

    test_shuffle(shuffle, input, expected);
}

#[test]
fn test_day22_inc() {
    let shuffle = &["deal with increment 3"];

    let input: &[usize] = &(0..10).collect::<Vec<_>>();
    let expected = &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3];

    test_shuffle(shuffle, input, expected);
}

#[test]
fn test_day22_case2() {
    let shuffle = &[
        "deal with increment 7",
        "deal into new stack",
        "deal into new stack",
    ];

    let input: &[usize] = &(0..10).collect::<Vec<_>>();
    let expected = &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7];

    test_shuffle(shuffle, input, expected);
}

#[cfg(test)]
fn shuffle_index_compounded(index: i128, deck_size: i128, shuffle: &[Shuffle]) -> i128 {
    let (a, b) = compound_shuffle(deck_size as i128, shuffle);
    println!("Compound shuffle ax + b: (a, b) = ({}, {})", a, b);
    (a * index + b).rem_euclid(deck_size)
}

#[test]
fn test_day22_part2_equals_part1() -> AdventResult<()> {
    let input = &get_input::<String>(22)?.first_column();
    let shuffles = input
        .iter()
        .map(parse_shuffle)
        .collect::<AdventResult<Vec<_>>>()?;
    let res = shuffle_index(2019, 10007, &shuffles);
    let res_compound = shuffle_index_compounded(2019, 10007, &shuffles);
    assert_eq!(res as i128, res_compound);
    Ok(())
}
