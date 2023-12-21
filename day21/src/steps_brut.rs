//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct XY {
    x: i64,
    y: i64,
}

impl XY {
    const fn new(x: i64, y: i64) -> XY { XY {x, y} }

    const fn add(&self, other: &XY) -> XY { XY { x: self.x + other.x, y: self.y + other.y } }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
use Direction::{UP, RIGHT, DOWN, LEFT};

impl Direction {
    const ALL: [Direction; 4] = [
        UP,
        RIGHT,
        DOWN,
        LEFT,
    ];

    //const fn as_entry(&self) -> usize {
    //    match self {
    //        UP => 0,
    //        RIGHT => 1,
    //        DOWN => 2,
    //        LEFT => 3,
    //    }
    //}

    const fn as_direction(&self) -> XY {
        match self {
            UP => XY::new(0, -1),
            RIGHT => XY::new(1, 0),
            DOWN => XY::new(0, 1),
            LEFT => XY::new(-1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Type {
    Rock,
    Garden,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Node {
    typ: Type,
}

impl Node {
    fn from_char(c: char) -> Node {
        let typ = match c {
            '.' => Type::Garden,
            '#' => Type::Rock,
            'S' => Type::Garden,
            _ => panic!("Wrong char!"),
        };
        Node {
            typ,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Map {
    nodes: Vec<Vec<Node>>,
    start: XY,
}

impl Map {
    fn from_input<I>(lines: I) -> Map
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();
        let mut start = XY::new(-1, -1);

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for (x, c) in line.char_indices() {
                nodes[y].push(Node::from_char(c));

                if c == 'S' {
                    start = XY::new(x as i64, y as i64);
                }
            }
        }

        Map {
            nodes,
            start,
        }
    }

    fn width(&self) -> usize { self.nodes[0].len() }
    fn height(&self) -> usize { self.nodes.len() }

    fn is_valid(&self, pos: &XY) -> bool {
        pos.x >= 0 && pos.y >= 0 &&
                pos.x < self.width() as i64 && pos.y < self.height() as i64
    }

    fn node_at(&self, at: &XY) -> &Node {
        if !self.is_valid(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        &self.nodes[at.y as usize][at.x as usize ]
    }

    fn neighbours(&self, at: &XY) -> Vec<XY> {
        let mut neighs = Vec::new();
        for d in &Direction::ALL {
            let potential = at.add(&d.as_direction());
            if !self.is_valid(&potential) {
                continue;
            }
            let node = self.node_at(&potential);
            if node.typ == Type::Rock {
                continue;
            }
            neighs.push(potential);
        }
        neighs
    }

    fn multi_node_at(&self, at: &XY) -> &Node {
        let max_x = self.width() as i64;
        let max_y = self.height() as i64;

        let x = if at.x >= 0 {
            at.x % max_x
        } else {
            (max_x + (at.x % max_x)) % max_x
        };

        let y = if at.y >= 0 {
            at.y % max_y
        } else {
            (max_y + (at.y % max_y)) % max_y
        };
        &self.nodes[y as usize][x as usize]
    }

    fn multi_neighbours(&self, at: &XY) -> Vec<XY> {
        let mut neighs = Vec::new();
        for d in &Direction::ALL {
            let potential = at.add(&d.as_direction());
            let node = self.multi_node_at(&potential);
            if node.typ == Type::Rock {
                continue;
            }
            neighs.push(potential);
        }
        neighs
    }

    fn find_all_in_dist(&self, goal_dist: i64) -> i64 {
        let mut curr_round = 0;
        let mut round_positions = HashSet::new();
        round_positions.insert(self.start);

        while curr_round < goal_dist {
            curr_round += 1;
            let positions: Vec<XY> = round_positions.into_iter().collect();
            round_positions = HashSet::new();

            for pos in positions {
                for n in self.multi_neighbours(&pos) {
                    round_positions.insert(n);
                }
            }
        }
        round_positions.len() as i64
    }

    #[allow(dead_code)]
    fn bfs(&self, start: XY) -> Vec<Vec<i64>> {
        let mut dists = vec![vec![i64::MAX; self.width()]; self.height()];

        let mut queue = VecDeque::new();
        queue.push_back((0, start));
        dists[start.y as usize][start.x as usize] = 0;

        while let Some((dist, pos)) = queue.pop_front() {
            if dist > dists[pos.y as usize][pos.x as usize] {
                continue;
            }
            let new_dist = dist + 1;
            for n in self.neighbours(&pos) {
                if dists[n.y as usize][n.x as usize] > new_dist {
                    dists[n.y as usize][n.x as usize] = new_dist;
                    queue.push_back((new_dist, n));
                }
            }
        }

        dists
    }

    fn multi_bfs(&self, max: i64) -> HashMap<XY, i64> {
        let mut dists = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((0, self.start));
        dists.insert(self.start, 0);

        while let Some((dist, pos)) = queue.pop_front() {
            if dist > max {
                continue;
            }
            let new_dist = dist + 1;
            for n in self.multi_neighbours(&pos) {
                if let Some(_) = dists.get(&n) {
                    continue;
                }
                dists.insert(n, new_dist);
                queue.push_back((new_dist, n));
            }
        }

        dists
    }
}

#[allow(dead_code)]
fn print_dists(dists: &Vec<Vec<i64>>) {
    for y in 0..dists.len() {
        for x in 0..dists[y].len() {
            let d = dists[y][x];
            if d == i64::MAX {
                print!("  #;");
            } else {
                print!("{: >3};", d);
            }
        }
        println!();
    }
}

fn count_end_positions(dists: &HashMap<XY, i64>, mod_2: i64) -> i64 {
    let mut count = 0;
    for (_, &dist) in dists {
        if dist % 2 == mod_2 {
            count += 1;
        }
    }

    count
}

fn count_end_positions_vec(dists: &Vec<Vec<i64>>, mod_2: i64, max: i64) -> i64 {
    let mut count = 0;
    for y in 0..dists.len() {
        for x in 0..dists[y].len() {
            let dist = dists[y][x];
            if dist < max && dist % 2 == mod_2 {
                count += 1;
            }
        }
    }

    count
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap()).peekable();

    let mut goal_steps = 26501365;
    if let Some(p) = lines.peek() {
        if let Ok(steps) = p.trim().parse::<i64>() {
            goal_steps = steps;
            let _ = lines.next();
        }
    }
    let goal_steps = goal_steps;

    let map = Map::from_input(lines);
    let ds = map.bfs(map.start);
    //let ds = map.multi_bfs(goal_steps);
    //
    //writeln!(output, "{}", count_end_positions(&ds, goal_steps)).unwrap();
    //print_dists(&ds);

    // This works only for the input file.
    let mid_dist = 65;
    let len = 131;

    let goal_mod_2 = goal_steps % 2;
    let count_for_even = count_end_positions_vec(&ds, goal_mod_2, i64::MAX);
    let count_for_odd = count_end_positions_vec(&ds, (goal_mod_2 + 1) % 2, i64::MAX);

    let full_tiles_arm = (goal_steps - mid_dist) / len - 1;
    let even_tiles = (full_tiles_arm) * (full_tiles_arm);
    let odd_tiles = (full_tiles_arm + 1) * (full_tiles_arm + 1);

    // Modality is because full_tiles_arm + 1 % 2 == 0
    let count_edge_left = count_end_positions_vec(&map.bfs(XY::new(130, 65)), (goal_mod_2 + 1) % 2, len);
    let count_edge_up = count_end_positions_vec(&map.bfs(XY::new(65, 130)), (goal_mod_2 + 1) % 2, len);
    let count_edge_right = count_end_positions_vec(&map.bfs(XY::new(0, 65)), (goal_mod_2 + 1) % 2, len);
    let count_edge_down = count_end_positions_vec(&map.bfs(XY::new(65, 0)), (goal_mod_2 + 1) % 2, len);
    println!("count_for_even: {}", count_for_even);
    println!("count_for_odd: {}", count_for_odd);
    println!("count_edge_left: {}", count_edge_left);
    println!("count_edge_up: {}", count_edge_up);
    println!("count_edge_right: {}", count_edge_right);
    println!("count_edge_down: {}", count_edge_down);
    let edge_tiles = full_tiles_arm * 4;

    let result = odd_tiles * count_for_odd  + even_tiles * count_for_even +
        count_edge_down + count_edge_up + count_edge_left + count_edge_right +
        (edge_tiles - 4) * count_edge_up;
    println!("Result: {}", result);
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[test]
    fn sample0() {
        test_ignore_whitespaces(
            "6
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........",
            "16",
        );
    }

    #[test]
    fn sample1() {
        test_ignore_whitespaces(
            "10
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........",
            "50",
        );
    }
    
    #[test]
    fn sample2() {
        test_ignore_whitespaces(
            "50
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........",
            "1594",
        );
    }
    
    #[test]
    fn sample3() {
        test_ignore_whitespaces(
            "100
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........",
            "6536",
        );
    }

    #[test]
    fn sample4() {
        test_ignore_whitespaces(
            "500
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........",
            "167004",
        );
    }

    #[test]
    fn sample5() {
        test_ignore_whitespaces(
            "1000
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........",
            "668697",
        );
    }

    #[test]
    fn sample6() {
        test_ignore_whitespaces(
            "5000
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........",
            "16733044",
        );
    }
}
