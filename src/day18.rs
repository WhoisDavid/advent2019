use advent2019::{get_input, AdventResult};
use pathfinding::directed::{bfs, dijkstra};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() -> AdventResult<()> {
    let input = get_input::<String>(18)?.first_column();
    let maze = &input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    solve_part1(maze)?;
    solve_part2(maze)?;
    Ok(())
}

fn solve_part1(input: &[Vec<char>]) -> AdventResult<()> {
    let maze = Maze::new(input);
    let res = maze
        .shortest_path_to_all_keys_dijkstra_pathfinding()
        .unwrap();
    println!("Shortest path to keys: {}", res);
    Ok(())
}

fn solve_part2(input: &[Vec<char>]) -> AdventResult<()> {
    let grid = &mut input.to_vec();
    let maze = Maze::new_part2(grid);
    let res = maze
        .shortest_path_to_all_keys_dijkstra_pathfinding_part2()
        .unwrap();

    println!("Shortest path to keys with 4 robots: {}", res);
    Ok(())
}

#[derive(Clone)]
struct Maze<'a> {
    grid: &'a [Vec<char>],
    start: Node,
    key_count: u32,
}

impl<'a> Maze<'a> {
    const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    fn new(grid: &'a [Vec<char>]) -> Self {
        let start = Self::start_point(grid).expect("Start point!");
        let key_count = grid
            .iter()
            .flat_map(|row| row.iter().filter(|c| c.is_ascii_lowercase()))
            .count() as u32;
        Self {
            grid,
            start,
            key_count,
        }
    }

    fn new_part2(grid: &'a mut [Vec<char>]) -> Self {
        let start = Self::start_point(grid).expect("Start point!");
        let key_count = grid
            .iter()
            .flat_map(|row| row.iter().filter(|c| c.is_ascii_lowercase()))
            .count() as u32;

        let start_node = Maze::start_point(grid).unwrap();
        let Node(x, y) = start_node;

        grid[x][y] = '#';
        grid[x][y - 1] = '#';
        grid[x][y + 1] = '#';
        grid[x - 1][y] = '#';
        grid[x + 1][y] = '#';

        Self {
            grid,
            start,
            key_count,
        }
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

    fn value(&self, pos: Node) -> char {
        self.grid[pos.0][pos.1]
    }

    fn key_to_bitmask(c: char) -> u32 {
        if c.is_ascii_alphabetic() {
            1 << (c.to_ascii_lowercase() as u8 - b'a')
        } else {
            0
        }
    }

    fn bfs_reachable_neighbors_homemade(
        &self,
        start_node: Node,
        adjacency_map: &mut HashMap<Node, Vec<(Node, u32, u32)>>,
    ) {
        let reachable_keys = adjacency_map.entry(start_node).or_default();

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let start_val = self.value(start_node);
        queue.push_front((start_node, 0u32, 0u32));

        while let Some((node, required_keys, distance)) = queue.pop_front() {
            let val = self.value(node);
            if val.is_ascii_lowercase() && val != start_val && val != '@' {
                reachable_keys.push((node, required_keys, distance));
                continue;
            }

            let neighbours = Self::DIRS
                .iter()
                .map(|(dx, dy)| {
                    Node(
                        (node.0 as isize + dx) as usize,
                        (node.1 as isize + dy) as usize,
                    )
                })
                .filter(|node| self.value(*node) != '#');

            for neighbour in neighbours {
                if visited.insert(neighbour) {
                    let val = self.value(neighbour);
                    let new_key_required = if val.is_ascii_uppercase() {
                        Self::key_to_bitmask(val)
                    } else {
                        0
                    };
                    queue.push_back((neighbour, required_keys | new_key_required, distance + 1))
                }
            }
        }
    }

    #[allow(dead_code)]
    fn shortest_path_to_all_keys_dijkstra_homemade(&self) -> Option<u32> {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        distances.insert((self.start, 0u32), 0u32);
        queue.push(std::cmp::Reverse((0u32, self.start, 0u32)));

        let adjacency_map = &mut HashMap::new();

        while let Some(std::cmp::Reverse((distance, node, collected_keys))) = queue.pop() {
            if collected_keys.count_ones() == self.key_count {
                return Some(distance);
            }

            if !visited.insert((node, collected_keys)) {
                // Already visited this node
                continue;
            }

            if !adjacency_map.contains_key(&node) {
                self.bfs_reachable_neighbors_homemade(node, adjacency_map);
            }

            for &(neighbor, required_keys, distance_to_neighbor) in adjacency_map[&node].iter() {
                if required_keys & collected_keys != required_keys {
                    continue;
                };
                let new_distance = distance + distance_to_neighbor;
                let is_shorter = distances
                    .get(&(neighbor, distance_to_neighbor))
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    let collected_keys =
                        collected_keys | Self::key_to_bitmask(self.value(neighbor));
                    distances.insert((neighbor, collected_keys), new_distance);
                    queue.push(std::cmp::Reverse((new_distance, neighbor, collected_keys)));
                }
            }
        }

        None
    }

    fn shortest_path_to_all_keys_dijkstra_pathfinding(&self) -> Option<u32> {
        let adjacency_map = &mut HashMap::new();

        dijkstra::dijkstra(
            &(self.start, 0u32),
            |&(node, keys): &(Node, u32)| {
                if !adjacency_map.contains_key(&node) {
                    self.bfs_reachable_neighbors_homemade(node, adjacency_map);
                }
                adjacency_map[&node]
                    .clone()
                    .into_iter()
                    .filter(move |(_, required_keys, _)| keys & required_keys == *required_keys)
                    .map(move |(node, _, distance)| {
                        (
                            (node, keys | Self::key_to_bitmask(self.value(node))),
                            distance,
                        )
                    })
            },
            |&(_, keys)| keys.count_ones() == self.key_count,
        )
        .map(|(_, shortest_path)| shortest_path)
    }

    #[allow(dead_code)]
    fn bfs_reachable_neigbhbors_pathfinding(
        &self,
        start_node: Node,
        adjacency_map: &mut HashMap<Node, Vec<DijkstraNode>>,
    ) {
        let reachable_keys = adjacency_map.entry(start_node).or_default();
        let start_val = self.value(start_node);

        bfs::bfs(
            &DijkstraNode(start_node, 0, 0),
            |&DijkstraNode(node, required_keys, distance)| {
                let val = self.value(node);
                let stop = val.is_ascii_lowercase() && val != start_val && val != '@';
                if stop {
                    reachable_keys.push(DijkstraNode(node, required_keys, distance));
                }

                Self::DIRS
                    .iter()
                    .map(move |(dx, dy)| {
                        Node(
                            (node.0 as isize + dx) as usize,
                            (node.1 as isize + dy) as usize,
                        )
                    })
                    .filter(move |_| !stop)
                    .filter(|neighbor| self.value(*neighbor) != '#')
                    .map(move |neighbor| {
                        let val = self.value(neighbor);
                        let new_key_required = if val.is_ascii_uppercase() {
                            Self::key_to_bitmask(val)
                        } else {
                            0
                        };
                        DijkstraNode(neighbor, required_keys | new_key_required, distance + 1)
                    })
            },
            |_| false,
        );
    }

    fn shortest_path_to_all_keys_dijkstra_pathfinding_part2(&self) -> Option<u32> {
        let adjacency_map = &mut HashMap::new();

        let Node(x, y) = self.start;

        let robots = [
            Node(x - 1, y - 1),
            Node(x - 1, y + 1),
            Node(x + 1, y - 1),
            Node(x + 1, y + 1),
        ];

        dijkstra::dijkstra(
            &(robots, 0u32),
            |&(robots, keys)| {
                let mut successors = Vec::with_capacity(8);

                for (robot_id, robot) in robots.iter().enumerate() {
                    if !adjacency_map.contains_key(robot) {
                        self.bfs_reachable_neighbors_homemade(*robot, adjacency_map);
                    }
                
                    let mut new_robots = robots;
                    successors.extend(
                        adjacency_map[robot]
                            .clone()
                            .into_iter()
                            .filter(move |(_, required_keys, _)| {
                                keys & required_keys == *required_keys
                            })
                            .map(move |(neighbor, _, distance)| {
                                new_robots[robot_id] = neighbor;
                                (
                                    (
                                        new_robots,
                                        keys | Self::key_to_bitmask(self.value(neighbor)),
                                    ),
                                    distance,
                                )
                            }),
                    );
                }

                successors
            },
            |&(_, keys)| keys.count_ones() == self.key_count,
        )
        .map(|(_, shortest_path)| shortest_path)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Node(usize, usize);

#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
struct DijkstraNode(Node, u32, u32);

impl PartialEq for DijkstraNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for DijkstraNode {}

impl std::hash::Hash for DijkstraNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(test)]
fn test_part1(input: &[&str], output: u32) {
    let input: Vec<_> = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let maze = Maze::new(&input);
    let res = maze.shortest_path_to_all_keys_dijkstra_pathfinding();
    assert_eq!(res, Some(output));
}

#[cfg(test)]
fn test_part2(input: &[&str], output: u32) {
    let mut input: Vec<_> = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let maze = Maze::new_part2(&mut input);
    let res = maze.shortest_path_to_all_keys_dijkstra_pathfinding_part2();
    assert_eq!(res, Some(output));
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
    test_part1(input, 136)
}

#[test]
fn test_day18_case1_part2() {
    let input = &[
        "#######", //
        "#a.#Cd#", //
        "##...##", //
        "##.@.##", //
        "##...##", //
        "#cB#Ab#", //
        "#######", //
    ];
    test_part2(input, 8)
}

#[test]
fn test_day18_case2_part2() {
    let input = &[
        "###############",
        "#d.ABC.#.....a#",
        "######...######",
        "######.@.######",
        "######...######",
        "#b.....#.....c#",
        "###############",
    ];
    test_part2(input, 24)
}

#[test]
fn test_day18_case3_part2() {
    let input = &[
        "#############",
        "#DcBa.#.GhKl#",
        "#.###...#I###",
        "#e#d#.@.#j#k#",
        "###C#...###J#",
        "#fEbA.#.FgHi#",
        "#############",
    ];
    test_part2(input, 32)
}

#[test]
fn test_day18_case4_part2() {
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
    test_part2(input, 72)
}
