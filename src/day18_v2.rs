use advent2019::{get_input, AdventResult};
use pathfinding::directed::dijkstra;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() -> AdventResult<()> {
    let input = &get_input::<String>(18)?.first_column();
    let maze = &input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    solve_part1(maze)?;
    // solve_part2_lucky(maze)?;
    Ok(())
}

fn solve_part1(input: &[Vec<char>]) -> AdventResult<()> {
    let maze = Maze::new(input);
    let res = maze.shortest_path_to_all_keys().unwrap();
    println!("Shortest path to keys: {}", res);
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

// fn solve_part2_lucky(grid: &[Vec<char>]) -> AdventResult<()> {
//     let (x, y) = Maze::start_point(grid).expect("@ start");
//     let mut grid = grid.to_vec();
//     // Substitute start point
//     grid[x][y] = '#';
//     grid[x][y - 1] = '#';
//     grid[x][y + 1] = '#';
//     grid[x - 1][y] = '#';
//     grid[x + 1][y] = '#';
//     grid[x - 1][y - 1] = '@';
//     grid[x + 1][y - 1] = '@';
//     grid[x - 1][y + 1] = '@';
//     grid[x + 1][y + 1] = '@';

//     // Remove doors
//     for row in grid.iter_mut() {
//         for c in row.iter_mut() {
//             if c.is_ascii_uppercase() {
//                 *c = '.'
//             }
//         }
//     }

//     // Split into 4 mazes
//     let maze1 = &grid[..=x]
//         .iter()
//         .map(|c| c.iter().take(y + 1).copied().collect::<Vec<_>>())
//         .collect::<Vec<_>>();
//     let maze2 = &grid[x..]
//         .iter()
//         .map(|c| c.iter().take(y + 1).copied().collect::<Vec<_>>())
//         .collect::<Vec<_>>();
//     let maze3 = &grid[..=x]
//         .iter()
//         .map(|c| c.iter().skip(y).copied().collect::<Vec<_>>())
//         .collect::<Vec<_>>();
//     let maze4 = &grid[x..]
//         .iter()
//         .map(|c| c.iter().skip(y).copied().collect::<Vec<_>>())
//         .collect::<Vec<_>>();
//     let maze1 = Maze::new(maze1);
//     let maze2 = Maze::new(maze2);
//     let maze3 = Maze::new(maze3);
//     let maze4 = Maze::new(maze4);

//     let maze = &[maze1, maze2, maze3, maze4];
//     let res: usize = maze
//         .iter()
//         .map(|maze| maze.shortest_path_bfs_crate().unwrap())
//         .sum();
//     println!("Shortest path part 2: {}", res);
//     Ok(())
// }

#[derive(Clone)]
struct Maze<'a> {
    grid: &'a [Vec<char>],
    start: Node,
}

impl<'a> Maze<'a> {
    const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    fn new(grid: &'a [Vec<char>]) -> Self {
        let start = Self::start_point(grid).expect("Start point!");
        Self { grid, start }
    }

    fn start_point(grid: &[Vec<char>]) -> Option<Node> {
        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '@' {
                    return Some(Node(i, j));
                }
            }
        }
        None
    }

    fn value(&self, pos: &Node) -> char {
        self.grid[pos.0][pos.1]
    }

    fn build_key_graph(&self) -> HashMap<char, HashMap<u32, Vec<(char, u32)>>> {
        let mut key_graph = HashMap::new();
        self.build_key_graph_aux(&self.start, &mut key_graph);

        let mut graph: HashMap<char, HashMap<u32, Vec<(char, u32)>>> = HashMap::new();
        for ((a, b), (required_keys, distance)) in key_graph {
            graph
                .entry(a)
                .or_default()
                .entry(required_keys)
                .or_default()
                .push((b, distance));
        }
        graph
    }

    fn key_to_bitmask(c: char) -> u32 {
        if c.is_ascii_alphabetic() {
            1 << (c.to_ascii_lowercase() as u8 - b'a')
        } else {
            0
        }
    }

    fn build_key_graph_aux(
        &self,
        start_node: &Node,
        key_graph: &mut HashMap<(char, char), (u32, u32)>,
    ) {
        let mut parents = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_front(start_node.clone());

        let start_val = self.value(&start_node);

        while let Some(node) = queue.pop_front() {
            let val = self.value(&node);
            if val.is_ascii_lowercase() && val != start_val && val != '@' {
                let key = (start_val, val);
                let sym_key = (val, start_val);

                if !key_graph.contains_key(&key) && !key_graph.contains_key(&sym_key) {
                    let mut distance = 0;
                    let mut required_keys = 0;
                    let mut node_tmp = &node;
                    while node_tmp != start_node {
                        distance += 1;
                        node_tmp = &parents[node_tmp];
                        let val = self.value(&node_tmp);
                        if val.is_ascii_uppercase() {
                            required_keys |= Self::key_to_bitmask(val);
                        }
                    }
                    key_graph.insert(key, (required_keys, distance));
                    self.build_key_graph_aux(&node, key_graph);
                    continue;
                }
            }

            let neighbours = Self::DIRS
                .iter()
                .map(|(dx, dy)| {
                    Node(
                        (node.0 as isize + dx) as usize,
                        (node.1 as isize + dy) as usize,
                    )
                })
                .filter(|node| self.value(node) != '#');

            for neighbour in neighbours {
                if !parents.contains_key(&neighbour) {
                    parents.insert(neighbour.clone(), node.clone());
                    queue.push_back(neighbour)
                }
            }
        }
    }

    #[allow(dead_code)]
    fn shortest_path_pathfinding_dijkstra(&self) -> Option<u32> {
        let key_graph = self.build_key_graph();
        let key_count = (key_graph.len() - 1) as u32;

        dijkstra::dijkstra(
            &('@', 0),
            |&(node, keys): &(char, u32)| {
                key_graph[&node]
                    .iter()
                    .filter(move |(required_keys, _)| keys & *required_keys == **required_keys)
                    .flat_map(|(_, neighbors)| neighbors)
                    .map(move |(node, distance)| {
                        ((*node, keys | Maze::key_to_bitmask(*node)), *distance)
                    })
            },
            |&(_, keys)| keys.count_ones() == key_count,
        )
        .map(|(_, shortest_path)| shortest_path)
    }

    fn shortest_path_to_all_keys(&self) -> Option<u32> {
        let key_graph = self.build_key_graph();
        let key_count = (key_graph.len() - 1) as u32;

        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();

        distances.insert(('@', 0u32), 0u32);
        to_visit.push(std::cmp::Reverse((0u32, ('@', 0u32))));

        while let Some(std::cmp::Reverse((distance, (node, keys)))) = to_visit.pop() {
            if keys.count_ones() == key_count {
                return Some(distance);
            }

            if !visited.insert((node, keys)) {
                // Already visited this node
                continue;
            }

            let neighbors = key_graph[&node]
                .iter()
                .filter(move |(required_keys, _)| keys & *required_keys == **required_keys)
                .flat_map(|(_, neighbors)| neighbors)
                .map(move |(node, distance)| {
                    ((*node, keys | Maze::key_to_bitmask(*node)), *distance)
                });

            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(neighbor, new_distance);
                    to_visit.push(std::cmp::Reverse((new_distance, neighbor)));
                }
            }
        }

        None
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Node(usize, usize);

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
    let maze = Maze::new(&input);
    // let res = maze.shortest_path_homemade_bfs();
    let res = maze.shortest_path_to_all_keys();

    assert_eq!(res, Some(136));
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
    let Node(x, y) = start;

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

    println!("{:?}", maze1.shortest_path_to_all_keys());
    println!("{:?}", maze2.shortest_path_to_all_keys());
    println!("{:?}", maze3.shortest_path_to_all_keys());
    println!("{:?}", maze4.shortest_path_to_all_keys());
    // let res = maze.shortest_path_bfs_part2();
    // let res = mazes.iter_mut().map(|maze| maze.shortest_path_bfs_crate().unwrap()).sum::<usize>();
    // let res = maze.shortest_path_bfs_crate();
    // assert_eq!(res, 72)
}
