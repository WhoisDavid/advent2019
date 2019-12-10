use crate::{get_input, AdventResult};
use num::rational::Ratio;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn solve_part1() -> AdventResult<usize> {
    let input = &get_input::<String>(10)?.first_column();
    let asteroid_map = read_map(input);
    let res = best_asteroid(&asteroid_map);
    println!("Output: {:?}", res);
    Ok(res.0)
}

pub fn solve_part2() -> AdventResult<()> {
    let input = &get_input::<String>(10)?.first_column();
    let asteroid_map = read_map(input);
    let best = best_asteroid(&asteroid_map);
    println!("Laser location: ({}, {})", best.1, best.2);
    let vis = closest_asteroids_by_angle(&asteroid_map, best.1, best.2, 100);
    println!("Mapped closest asteroids!");
    let nth = 200;
    let hits = laser_take_n(vis, nth);
    let res = hits[nth-1];
    println!("Output: {:?}", res);
    Ok(())
}

fn read_map<T: AsRef<str>>(asteroid_map: &[T]) -> Vec<Vec<bool>> {
    asteroid_map
        .iter()
        .map(|r| {
            r.as_ref()
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unexpected character!"),
                })
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn visible_asteroids_count(asteroid_map: &[Vec<bool>], pos_x: usize, pos_y: usize) -> usize {
    let mut seen_pos = HashSet::new();
    let mut seen_neg = HashSet::new();
    let mut vertical = (0, 0);
    for y in 0..asteroid_map.len() {
        for x in 0..asteroid_map[0].len() {
            if asteroid_map[y][x] {
                let pos = (x as isize - pos_x as isize, y as isize - pos_y as isize);
                match pos {
                    (0, 0) => (),
                    (0, y) if y > 0 => vertical.0 = 1,
                    (0, y) if y < 0 => vertical.1 = 1,
                    (x, y) if x > 0 => {
                        seen_pos.insert(Ratio::new(y, x));
                    }
                    (x, y) if x < 0 => {
                        seen_neg.insert(Ratio::new(y, x));
                    }
                    (_, _) => unreachable!(),
                };
            }
        }
    }
    seen_pos.len() + seen_neg.len() + vertical.0 + vertical.1
}

fn best_asteroid(asteroid_map: &[Vec<bool>]) -> (usize, usize, usize) {
    let mut sights = BinaryHeap::new();
    for y in 0..asteroid_map.len() {
        for x in 0..asteroid_map[0].len() {
            if asteroid_map[y][x] {
                sights.push((visible_asteroids_count(asteroid_map, x, y), x, y));
            }
        }
    }
    sights.pop().expect("Asteroids!")
}

fn integer_angle(v1: (isize, isize), v2: (isize, isize), precision: usize) -> usize {
    let v1 = (v1.0 as f64, v1.1 as f64);
    let v2 = (v2.0 as f64, v2.1 as f64);
    let mut angle = v2.1.atan2(v2.0) - v1.1.atan2(v1.0);
    angle *= 180.0 / std::f64::consts::PI;
    if angle < 0.0 {
        angle += 360.0;
    }

    (angle* (precision as f64)).round() as usize
}

fn closest_asteroids_by_angle(
    asteroid_map: &[Vec<bool>],
    pos_x: usize,
    pos_y: usize,
    angle_precision: usize,
) -> HashMap<usize, BinaryHeap<(isize, usize, usize)>> {
    let mut visible: HashMap<usize, BinaryHeap<(isize, usize, usize)>> = HashMap::new();

    // Pointing up
    let reference = (0, -1);

    for y in 0..asteroid_map.len() {
        for x in 0..asteroid_map[0].len() {
            if asteroid_map[y][x] {
                let pos = (x as isize - pos_x as isize, y as isize - pos_y as isize);
                let dist = pos.0.abs() + pos.1.abs();
                visible
                    .entry(integer_angle(reference, pos, angle_precision))
                    .or_default()
                    .push((-dist, x, y));
            }
        }
    }
    visible
}

fn laser_take_n(
    mut asteroids_by_angle: HashMap<usize, BinaryHeap<(isize, usize, usize)>>,
    n: usize,
) -> Vec<(usize, usize)> {
    let mut asteroids_by_angle: Vec<_> = asteroids_by_angle.iter_mut().collect();
    asteroids_by_angle.sort_by(|(angle1, _), (angle2, _)| angle1.cmp(angle2));
    let len = asteroids_by_angle.len();

    let mut res = Vec::new();
    let mut vec_idx = 0;
    for _ in 0..n {
        while asteroids_by_angle[vec_idx % len].1.is_empty() {
            vec_idx += 1;
        }
        let nth = asteroids_by_angle[vec_idx % len].1.pop().unwrap();
        vec_idx += 1;
        res.push((nth.1, nth.2));
    }
    res
}

#[test]
fn test_day10_case1() {
    let asteroids = &[".#..#", ".....", "#####", "....#", "...##"];
    let map = read_map(asteroids);
    assert_eq!(best_asteroid(&map), (8, 3, 4))
}

#[test]
fn test_day10() {
    let asteroids = &[
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##",
    ];
    let map = read_map(asteroids);
    let vis = closest_asteroids_by_angle(&map, 11, 13, 10);
    let hits = laser_take_n(vis, 300);
    assert_eq!(hits[1 - 1], (11, 12));
    assert_eq!(hits[2 - 1], (12, 1));
    assert_eq!(hits[3 - 1], (12, 2));
    assert_eq!(hits[10 - 1], (12, 8));
    assert_eq!(hits[20 - 1], (16, 0));
    assert_eq!(hits[50 - 1], (16, 9));
    assert_eq!(hits[100 - 1], (10, 16));
    assert_eq!(hits[199 - 1], (9, 6));
    assert_eq!(hits[200 - 1], (8, 2));
    assert_eq!(hits[201 - 1], (10, 9));
    assert_eq!(hits[299], (11, 1));
}

#[test]
fn test_day10_case2() {
    let asteroids = &[
        "......#.#.",
        "#..#.#....",
        "..#######.",
        ".#.#.###..",
        ".#..#.....",
        "..#....#.#",
        "#..#....#.",
        ".##.#..###",
        "##...#..#.",
        ".#....####",
    ];
    let map = read_map(asteroids);
    assert_eq!(best_asteroid(&map), (33, 5, 8))
}

#[test]
fn test_day10_case3() {
    let asteroids = &[
        ".#..#..###",
        "####.###.#",
        "....###.#.",
        "..###.##.#",
        "##.##.#.#.",
        "....###..#",
        "..#.#..#.#",
        "#..#.#.###",
        ".##...##.#",
        ".....#.#..",
    ];
    let map = read_map(asteroids);
    assert_eq!(best_asteroid(&map), (41, 6, 3))
}

#[test]
fn test_day10_case4() {
    let asteroids = &[
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##",
    ];
    let map = read_map(asteroids);
    assert_eq!(best_asteroid(&map), (210, 11, 13))
}
