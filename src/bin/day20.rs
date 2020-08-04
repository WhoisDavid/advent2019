use advent2019::{get_input, AdventResult};
use pathfinding::directed::bfs;
use std::collections::HashMap;

fn main() -> AdventResult<()> {
    let input = &get_input::<String>(20)?.first_column();
    let grid = &input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maze = &Maze::new(grid);
    solve_part1(maze)?;
    solve_part2(maze)?;
    Ok(())
}

fn solve_part1(maze: &Maze) -> AdventResult<()> {
    let res = maze.shortest_path(false).unwrap();
    println!("Shortest path: {}", res);
    Ok(())
}

#[allow(dead_code)]
fn solve_part2(maze: &Maze) -> AdventResult<()> {
    // let maze = Maze::new(input);
    let res = maze.shortest_path(true).unwrap();
    println!("Shortest path with levels: {}", res);
    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Portal {
    pos: (usize, usize),
    name: String,
    outer: bool,
}

impl Portal {
    fn new<T: AsRef<str>>(name: T, pos: (usize, usize), outer: bool) -> Self {
        Portal {
            pos,
            name: name.as_ref().to_string(),
            outer,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Loc {
    pos: (usize, usize),
    level: usize,
}

impl Loc {
    fn from_portal(p: &Portal) -> Self {
        Self {
            pos: p.pos,
            level: 0,
        }
    }
}

#[derive(Clone)]
struct Maze<'a> {
    grid: &'a [Vec<char>],
    start: Loc,
    end: Loc,
    portals: HashMap<(usize, usize), Portal>,
}

impl<'a> Maze<'a> {
    fn new(grid: &'a [Vec<char>]) -> Self {
        let mut portals = Self::portals(grid);

        let start = Loc::from_portal(&portals.remove("AA").expect("start")[0]);
        let end = Loc::from_portal(&portals.remove("ZZ").expect("end")[0]);

        let portals: HashMap<_, _> = portals
            .into_iter()
            .flat_map(|(_, mut doors)| {
                let p0 = doors.remove(0);
                let p1 = doors.remove(0);
                let p0_pos = p0.pos;
                let p1_pos = p1.pos;
                vec![(p0_pos, p1), (p1_pos, p0)]
            })
            .collect();

        Self {
            grid,
            start,
            end,
            portals,
        }
    }

    fn portals(grid: &[Vec<char>]) -> HashMap<String, Vec<Portal>> {
        let mut portals: HashMap<String, Vec<Portal>> = HashMap::new();
        for (i, row) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
            for (j, c) in row.iter().enumerate().skip(1).take(grid[0].len() - 2) {
                if c.is_ascii_uppercase() {
                    let is_outer =
                        i == 1 || i == grid.len() - 2 || j == 1 || j == grid[0].len() - 2;
                    // Vertical portal - point down
                    if grid[i - 1][j].is_ascii_uppercase() && grid[i + 1][j] == '.' {
                        let name = format!("{}{}", grid[i - 1][j], c);
                        let portal = Portal::new(name.clone(), (i + 1, j), is_outer);
                        portals.entry(name).or_default().push(portal);
                        continue;
                    }
                    // Vertical portal - point up
                    if grid[i - 1][j] == '.' && grid[i + 1][j].is_ascii_uppercase() {
                        let name = format!("{}{}", c, grid[i + 1][j]);
                        let portal = Portal::new(name.clone(), (i - 1, j), is_outer);
                        portals.entry(name).or_default().push(portal);
                        continue;
                    }
                    // Horizontal portal - point left
                    if grid[i][j - 1] == '.' && grid[i][j + 1].is_ascii_uppercase() {
                        let name = format!("{}{}", c, grid[i][j + 1]);
                        let portal = Portal::new(name.clone(), (i, j - 1), is_outer);
                        portals.entry(name).or_default().push(portal);
                        continue;
                    }
                    // Horizontal portal - point right
                    if grid[i][j - 1].is_ascii_uppercase() && grid[i][j + 1] == '.' {
                        let name = format!("{}{}", grid[i][j - 1], c);
                        let portal = Portal::new(name.clone(), (i, j + 1), is_outer);
                        portals.entry(name).or_default().push(portal);
                        continue;
                    }
                }
            }
        }
        portals
    }

    fn value(&self, loc: &Loc) -> char {
        self.grid[loc.pos.0][loc.pos.1]
    }

    fn check_position(&self, loc: &Loc, with_levels: bool) -> bool {
        if with_levels {
            if let Some(dst_portal) = self.portals.get(&loc.pos) {
                if !dst_portal.outer && loc.level == 0 {
                    return false;
                }
            }
        }

        self.value(&loc) == '.'
    }

    fn neighbours(&self, loc: &Loc, with_levels: bool) -> Vec<Loc> {
        let mut neighbours = Vec::with_capacity(5);

        let pos = loc.pos;
        if pos.0 > 2 {
            neighbours.push(Loc {
                pos: (pos.0 - 1, pos.1),
                level: loc.level,
            });
        }

        if pos.1 > 2 {
            neighbours.push(Loc {
                pos: (pos.0, pos.1 - 1),
                level: loc.level,
            });
        }

        if pos.0 < self.grid.len() - 3 {
            neighbours.push(Loc {
                pos: (pos.0 + 1, pos.1),
                level: loc.level,
            });
        }

        if pos.1 < self.grid[0].len() - 3 {
            neighbours.push(Loc {
                pos: (pos.0, pos.1 + 1),
                level: loc.level,
            });
        }

        if let Some(dst_portal) = self.portals.get(&pos) {
            let mut level = loc.level;
            if with_levels {
                // `!outer` because this is the opposite portal
                if !dst_portal.outer {
                    // Outer portal
                    level -= 1
                } else {
                    // Inner portal
                    level += 1
                }
            }

            neighbours.push(Loc {
                pos: dst_portal.pos,
                level,
            });
        }

        neighbours
            .into_iter()
            .filter(|p| self.check_position(p, with_levels))
            .collect()
    }

    fn shortest_path(&self, with_levels: bool) -> Option<usize> {
        let start = &self.start;
        let shortest_path_opt = bfs::bfs(
            start,
            |pos| self.neighbours(pos, with_levels),
            |pos| *pos == self.end,
        );
        shortest_path_opt.map(|s| s.len() - 1)
    }
}

#[test]
fn test_day20_case1() {
    let input = &[
        "                   A               ",
        "                   A               ",
        "  #################.#############  ",
        "  #.#...#...................#.#.#  ",
        "  #.#.#.###.###.###.#########.#.#  ",
        "  #.#.#.......#...#.....#.#.#...#  ",
        "  #.#########.###.#####.#.#.###.#  ",
        "  #.............#.#.....#.......#  ",
        "  ###.###########.###.#####.#.#.#  ",
        "  #.....#        A   C    #.#.#.#  ",
        "  #######        S   P    #####.#  ",
        "  #.#...#                 #......VT",
        "  #.#.#.#                 #.#####  ",
        "  #...#.#               YN....#.#  ",
        "  #.###.#                 #####.#  ",
        "DI....#.#                 #.....#  ",
        "  #####.#                 #.###.#  ",
        "ZZ......#               QG....#..AS",
        "  ###.###                 #######  ",
        "JO..#.#.#                 #.....#  ",
        "  #.#.#.#                 ###.#.#  ",
        "  #...#..DI             BU....#..LF",
        "  #####.#                 #.#####  ",
        "YN......#               VT..#....QG",
        "  #.###.#                 #.###.#  ",
        "  #.#...#                 #.....#  ",
        "  ###.###    J L     J    #.#.###  ",
        "  #.....#    O F     P    #.#...#  ",
        "  #.###.#####.#.#####.#####.###.#  ",
        "  #...#.#.#...#.....#.....#.#...#  ",
        "  #.#####.###.###.#.#.#########.#  ",
        "  #...#.#.....#...#.#.#.#.....#.#  ",
        "  #.###.#####.###.###.#.#.#######  ",
        "  #.#.........#...#.............#  ",
        "  #########.###.###.#############  ",
        "           B   J   C               ",
        "           U   P   P               ",
    ];
    let input: Vec<_> = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let maze = Maze::new(&input);
    let res = maze.shortest_path(false);

    assert_eq!(res, Some(58))
}

#[test]
fn test_day20_case1_part2() {
    let input = &[
        "             Z L X W       C                 ",
        "             Z P Q B       K                 ",
        "  ###########.#.#.#.#######.###############  ",
        "  #...#.......#.#.......#.#.......#.#.#...#  ",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ",
        "  #.###.#######.###.###.#.###.###.#.#######  ",
        "  #...#.......#.#...#...#.............#...#  ",
        "  #.#########.#######.#.#######.#######.###  ",
        "  #...#.#    F       R I       Z    #.#.#.#  ",
        "  #.###.#    D       E C       H    #.#.#.#  ",
        "  #.#...#                           #...#.#  ",
        "  #.###.#                           #.###.#  ",
        "  #.#....OA                       WB..#.#..ZH",
        "  #.###.#                           #.#.#.#  ",
        "CJ......#                           #.....#  ",
        "  #######                           #######  ",
        "  #.#....CK                         #......IC",
        "  #.###.#                           #.###.#  ",
        "  #.....#                           #...#.#  ",
        "  ###.###                           #.#.#.#  ",
        "XF....#.#                         RF..#.#.#  ",
        "  #####.#                           #######  ",
        "  #......CJ                       NM..#...#  ",
        "  ###.#.#                           #.###.#  ",
        "RE....#.#                           #......RF",
        "  ###.###        X   X       L      #.#.#.#  ",
        "  #.....#        F   Q       P      #.#.#.#  ",
        "  ###.###########.###.#######.#########.###  ",
        "  #.....#...#.....#.......#...#.....#.#...#  ",
        "  #####.#.###.#######.#######.###.###.#.#.#  ",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  ",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  ",
        "  #.......#.....#.#...#...............#...#  ",
        "  #############.#.#.###.###################  ",
        "               A O F   N                     ",
        "               A A D   M                     ",
    ];
    let input: Vec<_> = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let maze = Maze::new(&input);
    let res = maze.shortest_path(true);

    assert_eq!(res, Some(396))
}
