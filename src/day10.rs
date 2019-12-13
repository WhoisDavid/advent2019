use advent2019::{get_input, AdventResult};
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() -> AdventResult<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}

pub fn solve_part1() -> AdventResult<usize> {
    let input = &get_input::<String>(10)?.first_column();
    let asteroid_map = read_map(input);
    let asteroids = map_to_asteroid_vec(asteroid_map);
    let (visible_asteroids, loc) = best_asteroid(&asteroids);
    println!("Best asteroid: {} asteroids are visible from asteroid ({},{})", loc.x, loc.y, visible_asteroids);
    Ok(visible_asteroids)
}

pub fn solve_part2() -> AdventResult<()> {
    let input = &get_input::<String>(10)?.first_column();
    let asteroid_map = read_map(input);
    let asteroids = map_to_asteroid_vec(asteroid_map);
    let best = best_asteroid(&asteroids);
    println!("Laser location: ({}, {})", best.1.x, best.1.y);
    let vis = closest_asteroids_by_angle(&asteroids, best.1, 100);
    println!("Mapped closest asteroids!");
    let nth = 200;
    let hits = get_laser_targets(vis);
    let res = &hits[nth - 1];
    println!("200th asteroids destroyed: ({}, {})", res.x, res.y);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AsteroidVector {
    x: isize,
    y: isize,
}

impl std::ops::Sub for &AsteroidVector {
    type Output = AsteroidVector;
    fn sub(self, other: &AsteroidVector) -> AsteroidVector {
        AsteroidVector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
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

fn map_to_asteroid_vec(asteroid_map: Vec<Vec<bool>>) -> Vec<AsteroidVector> {
    let mut asteroids = Vec::new();
    for (y, row) in asteroid_map.into_iter().enumerate() {
        for (x, is_asteroid) in row.into_iter().enumerate() {
            if is_asteroid {
                asteroids.push(AsteroidVector {
                    x: x as isize,
                    y: y as isize,
                })
            }
        }
    }
    asteroids
}

fn integer_angle(v1: &AsteroidVector, v2: &AsteroidVector, precision: usize) -> usize {
    let v1 = (v1.x as f64, v1.y as f64);
    let v2 = (v2.x as f64, v2.y as f64);
    let mut angle = v2.1.atan2(v2.0) - v1.1.atan2(v1.0);
    angle *= 180.0 / std::f64::consts::PI;
    if angle < 0.0 {
        angle += 360.0;
    }

    (angle * (precision as f64)).round() as usize
}

fn visible_asteroids_count(
    asteroids: &[AsteroidVector],
    current_asteroid: &AsteroidVector,
    angle_precision: usize,
) -> usize {
    let reference_vector = &AsteroidVector { x: 0, y: -1 };
    let mut seen_angles = HashSet::new();
    for asteroid in asteroids {
        if asteroid == current_asteroid {
            continue;
        }

        let vector = asteroid - current_asteroid;
        seen_angles.insert(integer_angle(reference_vector, &vector, angle_precision));
    }
    seen_angles.len()
}

fn best_asteroid(asteroids: &[AsteroidVector]) -> (usize, &AsteroidVector) {
    let mut sights = BinaryHeap::new();
    for asteroid in asteroids {
        sights.push((visible_asteroids_count(asteroids, asteroid, 10), asteroid));
    }
    sights.pop().expect("Asteroids!")
}

fn closest_asteroids_by_angle<'a>(
    asteroids: &'a [AsteroidVector],
    center: &AsteroidVector,
    angle_precision: usize,
) -> HashMap<usize, BinaryHeap<(isize, &'a AsteroidVector)>> {
    // Pointing up
    let reference_vector = &AsteroidVector { x: 0, y: -1 };
    let mut angles_map: HashMap<usize, BinaryHeap<(isize, &AsteroidVector)>> = HashMap::new();

    for asteroid in asteroids {
        if asteroid == center {
            continue;
        }

        let vector = asteroid - center;
        let distance = vector.x.abs() + vector.y.abs();
        let angle = integer_angle(reference_vector, &vector, angle_precision);
        angles_map
            .entry(angle)
            .or_default()
            .push((-distance, asteroid));
    }
    angles_map
}

fn get_laser_targets(
    mut asteroids_by_angle: HashMap<usize, BinaryHeap<(isize, &AsteroidVector)>>,
) -> Vec<&AsteroidVector> {
    let mut asteroids_by_angle: Vec<_> = asteroids_by_angle.iter_mut().collect();
    asteroids_by_angle.sort_by(|(angle1, _), (angle2, _)| angle1.cmp(angle2));
    let mut res = Vec::new();
    let mut vec_idx = 0;
    while !asteroids_by_angle.is_empty() {
        let len = asteroids_by_angle.len();
        let nth = asteroids_by_angle[vec_idx % len].1.pop();
        match nth {
            Some((_, asteroid)) => {
                res.push(asteroid);
                vec_idx += 1;
            }
            None => {
                asteroids_by_angle.remove(vec_idx % len);
            }
        }
    }
    res
}

#[test]
fn test_day10_case1() {
    let asteroids = &[".#..#", ".....", "#####", "....#", "...##"];
    let map = read_map(asteroids);
    let asteroids = map_to_asteroid_vec(map);
    assert_eq!(
        best_asteroid(&asteroids),
        (8, &AsteroidVector { x: 3, y: 4 })
    )
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
    let asteroids = map_to_asteroid_vec(map);
    assert_eq!(
        best_asteroid(&asteroids),
        (33, &AsteroidVector { x: 5, y: 8 })
    )
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
    let asteroids = map_to_asteroid_vec(map);
    assert_eq!(
        best_asteroid(&asteroids),
        (41, &AsteroidVector { x: 6, y: 3 })
    )
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
    let asteroids = map_to_asteroid_vec(map);
    assert_eq!(
        best_asteroid(&asteroids),
        (210, &AsteroidVector { x: 11, y: 13 })
    )
}

#[test]
fn test_day10_part2() {
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
    let asteroids = map_to_asteroid_vec(map);
    let center = AsteroidVector { x: 11, y: 13 };
    let vis = closest_asteroids_by_angle(&asteroids, &center, 100);
    let hits = get_laser_targets(vis);
    assert_eq!(hits[0], &AsteroidVector { x: 11, y: 12 });
    assert_eq!(hits[1], &AsteroidVector { x: 12, y: 1 });
    assert_eq!(hits[2], &AsteroidVector { x: 12, y: 2 });
    assert_eq!(hits[9], &AsteroidVector { x: 12, y: 8 });
    assert_eq!(hits[19], &AsteroidVector { x: 16, y: 0 });
    assert_eq!(hits[49], &AsteroidVector { x: 16, y: 9 });
    assert_eq!(hits[99], &AsteroidVector { x: 10, y: 16 });
    assert_eq!(hits[198], &AsteroidVector { x: 9, y: 6 });
    assert_eq!(hits[209], &AsteroidVector { x: 8, y: 2 });
    assert_eq!(hits[200], &AsteroidVector { x: 10, y: 9 });
    assert_eq!(hits[298], &AsteroidVector { x: 11, y: 1 });
}
