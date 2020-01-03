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
    solve_part2_lucky(maze)?;
    Ok(())
}

fn solve_part1(input: &[Vec<char>]) -> AdventResult<()> {
    let maze = Maze::new(input);
    // let res = maze.shortest_path();
    // let res = maze.shortest_path_helper(maze.start, 0, Dir::None);
    let res = maze.shortest_path_bfs_crate().unwrap();
    println!("Shortest path to keys: {}", res);
    // println!("Shortest path to keys: {:?}", maze.parent);
    Ok(())
}

/*
fn solve_part2(input: &[Vec<char>]) -> AdventResult<()> {
    let mut input = input.to_vec();
    let (j, i) = Maze::start_point(&input).unwrap();

    // let maze = Maze::new(input);
    // let res = maze.shortest_path();
    // println!("Shortest path to keys: {}", res);
    Ok(())
}
*/

fn solve_part2_lucky(grid: &[Vec<char>]) -> AdventResult<()> {
    let (x, y) = Maze::start_point(grid).expect("@ start");
    let mut grid = grid.to_vec();
    // Substitute start point
    grid[x][y] = '#';
    grid[x][y - 1] = '#';
    grid[x][y + 1] = '#';
    grid[x - 1][y] = '#';
    grid[x + 1][y] = '#';
    grid[x - 1][y - 1] = '@';
    grid[x + 1][y - 1] = '@';
    grid[x - 1][y + 1] = '@';
    grid[x + 1][y + 1] = '@';

    // Remove doors
    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            if c.is_ascii_uppercase() {
                *c = '.'
            }
        }
    }

    // Split into 4 mazes
    let maze1 = &grid[..=x]
        .iter()
        .map(|c| c.iter().take(y + 1).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze2 = &grid[x..]
        .iter()
        .map(|c| c.iter().take(y + 1).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze3 = &grid[..=x]
        .iter()
        .map(|c| c.iter().skip(y).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze4 = &grid[x..]
        .iter()
        .map(|c| c.iter().skip(y).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze1 = Maze::new(maze1);
    let maze2 = Maze::new(maze2);
    let maze3 = Maze::new(maze3);
    let maze4 = Maze::new(maze4);

    let maze = &[maze1, maze2, maze3, maze4];
    let res: usize = maze
        .iter()
        .map(|maze| maze.shortest_path_bfs_crate().unwrap())
        .sum();
    println!("Shortest path part 2: {}", res);
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

    fn neighbours(&self, node: &Node) -> Vec<Node> {
        let mut neighbours = Vec::with_capacity(4);        

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
            .map(|pos| node.clone_keys(pos))
            .filter_map(|node| self.check_position(node))
            .collect()
    }

    #[allow(dead_code)]
    fn shortest_path_homemade_bfs(&mut self) -> Option<usize> {
        let mut parents = HashMap::new();
        let start = Node::new(self.start);
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
                if !parents.contains_key(&neighbour) {
                    parents.insert(neighbour.clone(), node.clone());
                    queue.push_back(neighbour)
                }
            }
        }
        None
    }

    fn shortest_path_bfs_crate(&self) -> Option<usize> {
        let start = Node::new(self.start);
        let shortest_path_opt = bfs::bfs(
            &start,
            |node| self.neighbours(node),
            |node| node.keys.len() == self.keys,
        );
        shortest_path_opt.map(|s| s.len() - 1)
    }

    // fn check_robot_position(&self, robots: &Robots, robot_idx: usize, node: &Node) -> Robots {
    //     let mut robots = robots.clone();
    //     match self.value(node.pos) {
    //             '#' => None,
    //             '.' | '@' => Some(node),
    //             key if key.is_ascii_lowercase() => {
    //                 if !robots.keys.contains(&key) {
    //                     robots.keys.push(key);
    //                     robots.keys.sort();
    //                 }
    //                 robots.nodes[robot_idx] = node;
    //                 Some(Robots::new(node)
    //             }
    //             door if door.is_ascii_uppercase() => {
    //                 let key = &door.to_ascii_lowercase();
    //                 if robots.keys.contains(key) {
    //                     Some(node)
    //                 } else {
    //                     None
    //                 }
    //             }
    //             _ => panic!("Unexpected character!"),
    //         })
    //         .filter_map(|node| node)
    //         .collect();
    //     robots.nodes = nodes;
    //     robots
    // }

    // fn robots_neighbours(&self, robots: &Robots) -> Vec<Robots> {
    //     let mut neighbours = Vec::new();
    //     let next_robots = Vec::new();
    //     let node = robots.nodes[robot_idx];
    //     let pos = node.pos;
    //     if pos.0 > 0 {
    //         neighbours.push((pos.0 - 1, pos.1));
    //     }

    //     if pos.1 > 0 {
    //         neighbours.push((pos.0, pos.1 - 1));
    //     }

    //     if pos.0 < self.grid.len() - 1 {
    //         neighbours.push((pos.0 + 1, pos.1));
    //     }

    //     if pos.1 < self.grid[0].len() - 1 {
    //         neighbours.push((pos.0, pos.1 + 1));
    //     }

    //     neighbours
    //         .into_iter()
    //         .map(|pos| Node::new(pos))
    //         .map(|node| self.check_robot_position(robots, robot_idx, node))
    //         .filter_map(|node| node)
    //         .collect()
    // }

    // fn shortest_path_bfs_part2(&mut self) -> Option<usize> {
    //     let (x, y) = self.start;
    //     let mut robots = Robots::new(vec![
    //         Node::new((x - 1, y - 1)),
    //         Node::new((x + 1, y - 1)),
    //         Node::new((x - 1, y + 1)),
    //         Node::new((x + 1, y + 1)),
    //     ]);

    //     robots.running = Some(0);

    //     let shortest_path_opt = bfs::bfs(
    //         &robots,
    //         |robots| self.robots_neighbours(robots),
    //         |robots| robots.keys.len() == self.keys,
    //         );
    //     shortest_path_opt.map(|s| s.len() - 1)
    // }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Node {
    pos: (usize, usize),
    keys: Vec<char>,
}

impl Node {
    fn new(pos: (usize, usize)) -> Self {
        Node {
            pos,
            keys: Vec::with_capacity(26),
        }
    }

    fn clone_keys(&self, pos: (usize, usize)) -> Self {
        Node {
            pos,
            keys: self.keys.clone(),
        }
    }
}

/*
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Robots {
    nodes: Vec<Node>,
    keys: Vec<char>,
    running: Option<usize>,
}

impl Robots {
    fn new(nodes: Vec<Node>) -> Self {
        Robots {
            nodes,
            keys: Vec::new(),
            running: None,
        }
    }

    fn collect_and_distribute_keys(&mut self) {
        let mut keys: Vec<_> = self
            .nodes
            .iter()
            .flat_map(|robot| robot.keys.clone())
            .collect();
        keys.sort();
        keys.dedup();
        // Distribute keys to robots
        self.nodes
            .iter_mut()
            .for_each(|node| node.keys = keys.clone());
        self.keys = keys;
    }
}
*/

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
    let res = maze.shortest_path_homemade_bfs();
    // let res = maze.shortest_path_bfs_crate();

    assert_eq!(res, Some(136))
}

#[test]
fn test_day18_case1_part2() {
    let input = &[
        "#############",
        "#g#f.D#..h#l#",
        "#F###e#E###.#",
        "#dCba...BcIJ#",
        "#####.@.#####",
        "#nK.L...G...#",
        "#M###N#H###.#",
        "#o#m..#i#jk.#",
        "#############",
    ];
    let mut grid: Vec<_> = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();

    let start = Maze::start_point(&grid).expect("Start point!");
    let (x, y) = start;

    grid[x][y] = '#';
    grid[x][y - 1] = '#';
    grid[x][y + 1] = '#';
    grid[x - 1][y] = '#';
    grid[x + 1][y] = '#';
    grid[x - 1][y - 1] = '@';
    grid[x + 1][y - 1] = '@';
    grid[x - 1][y + 1] = '@';
    grid[x + 1][y + 1] = '@';

    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            if c.is_ascii_uppercase() {
                *c = '.'
            }
        }
    }

    // for row in &grid {
    //     println!("{:?}", row);
    // }
    println!("Start: {} {}", x, y);
    let maze1 = &grid[..=x]
        .iter()
        .map(|c| c.iter().take(y + 1).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze2 = &grid[x..]
        .iter()
        .map(|c| c.iter().take(y + 1).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze3 = &grid[..=x]
        .iter()
        .map(|c| c.iter().skip(y).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze4 = &grid[x..]
        .iter()
        .map(|c| c.iter().skip(y).copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze1 = Maze::new(maze1);
    let maze2 = Maze::new(maze2);
    let maze3 = Maze::new(maze3);
    let maze4 = Maze::new(maze4);

    // let mut mazes = [maze1, maze2, maze3, maze4];

    println!("{:?}", maze1.shortest_path_bfs_crate());
    println!("{:?}", maze2.shortest_path_bfs_crate());
    println!("{:?}", maze3.shortest_path_bfs_crate());
    println!("{:?}", maze4.shortest_path_bfs_crate());
    // let res = maze.shortest_path_bfs_part2();
    // let res = mazes.iter_mut().map(|maze| maze.shortest_path_bfs_crate().unwrap()).sum::<usize>();
    // let res = maze.shortest_path_bfs_crate();
    // assert_eq!(res, 72)
}
