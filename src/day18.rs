use advent2019::{get_input, AdventResult};
use pathfinding::directed::bfs;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() -> AdventResult<()> {
    let input = &get_input::<String>(18)?.first_column();
    let maze = &input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    solve_part1(maze)?;
    solve_part2(maze)?;
    Ok(())
}

fn solve_part1(input: &[Vec<char>]) -> AdventResult<()> {
    let mut maze = Maze::new(input);
    // let res = maze.shortest_path();
    // let res = maze.shortest_path_helper(maze.start, 0, Dir::None);
    let res = maze.shortest_path_bfs_crate().unwrap();
    println!("Shortest path to keys: {}", res);
    // println!("Shortest path to keys: {:?}", maze.parent);
    Ok(())
}

fn solve_part2(input: &[Vec<char>]) -> AdventResult<()> {
    let mut input = input.to_vec();
    let (j, i) = Maze::start_point(&input).unwrap();
    input[i][j] = '#';
    input[i][j - 1] = '#';
    input[i][j + 1] = '#';
    input[i - 1][j] = '#';
    input[i + 1][j] = '#';

    // let maze = Maze::new(input);
    // let res = maze.shortest_path();
    // println!("Shortest path to keys: {}", res);
    Ok(())
}

#[derive(Clone)]
struct Maze<'a> {
    grid: &'a [Vec<char>],
    start: (usize, usize),
    keys: usize,
}

impl<'a> Maze<'a> {
    fn new(grid: &'a [Vec<char>]) -> Self {
        let start = Self::start_point(grid).expect("Start point!");
        let keys = grid
            .iter()
            .flat_map(|row| row.iter().filter(|c| c.is_ascii_lowercase()))
            .count();

        Self { grid, start, keys }
    }

    fn start_point(grid: &[Vec<char>]) -> Option<(usize, usize)> {
        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '@' {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn value(&self, pos: (usize, usize)) -> char {
        self.grid[pos.0][pos.1]
    }

    fn neighbours(&self, node: &Node) -> Vec<Node> {
        let mut neighbours = Vec::new();
        let pos = node.pos;
        if pos.0 > 0 {
            neighbours.push((pos.0 - 1, pos.1));
        }

        if pos.1 > 0 {
            neighbours.push((pos.0, pos.1 - 1));
        }

        if pos.0 < self.grid.len() - 1 {
            neighbours.push((pos.0 + 1, pos.1));
        }

        if pos.1 < self.grid[0].len() - 1 {
            neighbours.push((pos.0, pos.1 + 1));
        }

        neighbours
            .into_iter()
            .map(|pos| Node {
                pos,
                keys: node.keys.clone(),
            })
            .collect()
    }

    fn check_position(&self, mut node: Node) -> Option<Node> {
        match self.value(node.pos) {
            '#' => None,
            '.' | '@' => Some(node),
            key if key.is_ascii_lowercase() => {
                if !node.keys.contains(&key) {
                    node.keys.push(key);
                    node.keys.sort();
                }
                Some(node)
            }
            door if door.is_ascii_uppercase() => {
                let key = &door.to_ascii_lowercase();
                if node.keys.contains(key) {
                    Some(node)
                } else {
                    None
                }
            }
            _ => panic!("Unexpected character!"),
        }
    }

    #[allow(dead_code)]
    fn shortest_path_homemade_bfs(&mut self) -> Option<usize> {
        let mut parents = HashMap::new();
        let start = Node {
            pos: self.start,
            keys: Vec::new(),
        };
        println!("Start: {:?}", start);
        let mut queue = VecDeque::new();
        queue.push_front(start.clone());
        while let Some(node) = queue.pop_front() {
            if node.keys.len() == self.keys {
                println!("Done!");
                let mut shortest_path = 0;
                let mut target = Some(&node);
                while let Some(node) = target {
                    if node == &start {
                        break;
                    }
                    shortest_path += 1;
                    target = parents.get(node);
                }
                return Some(shortest_path);
            }

            for neighbour in self.neighbours(&node) {
                if let Some(neighbour) = self.check_position(neighbour) {
                    if !parents.contains_key(&neighbour) {
                        parents.insert(neighbour.clone(), node.clone());
                        queue.push_back(neighbour)
                    }
                }
            }
        }
        None
    }

    fn valid_neighbours(&self, node: &Node) -> Vec<Node> {
        self.neighbours(node)
            .into_iter()
            .map(|node| self.check_position(node))
            .filter_map(|node| node)
            .collect()
    }

    fn shortest_path_bfs_crate(&mut self) -> Option<usize> {
        let start = Node {
            pos: self.start,
            keys: Vec::new(),
        };
        println!("Start: {:?}", start);
        let mut queue = VecDeque::new();
        queue.push_front(start.clone());

        let shortest_path_opt = bfs::bfs(
            &start,
            |node| self.valid_neighbours(node),
            |node| node.keys.len() == self.keys,
        );
        shortest_path_opt.map(|s| s.len() - 1)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Node {
    pos: (usize, usize),
    keys: Vec<char>,
}

#[test]
fn test_day18_case1() {
    let input = &[
        "#################",
        "#i.G..c...e..H.p#",
        "########.########",
        "#j.A..b...f..D.o#",
        "########@########",
        "#k.E..a...g..B.n#",
        "########.########",
        "#l.F..d...h..C.m#",
        "#################",
    ];
    let input: Vec<_> = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let mut maze = Maze::new(&input);
    // let res = maze.shortest_path_homemade_bfs();
    let res = maze.shortest_path_bfs_crate();

    assert_eq!(res, Some(136))
}
