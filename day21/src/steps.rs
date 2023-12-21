//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;

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

    fn is_valid(&self, pos: &XY) -> bool {
        pos.x >= 0 && pos.y >= 0 &&
                pos.x < self.nodes[0].len() as i64 && pos.y < self.nodes.len() as i64
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

    fn find_all_in_dist(&self, goal_dist: i64) -> i64 {
        let mut curr_round = 0;
        let mut round_positions = HashSet::new();
        round_positions.insert(self.start);

        while curr_round < goal_dist {
            curr_round += 1;
            let positions: Vec<XY> = round_positions.into_iter().collect();
            round_positions = HashSet::new();

            for pos in positions {
                for n in self.neighbours(&pos) {
                    round_positions.insert(n);
                }
            }
        }
        round_positions.len() as i64
    }
}


fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap()).peekable();

    let mut goal_steps = 64;
    if let Some(p) = lines.peek() {
        if let Ok(steps) = p.trim().parse::<i64>() {
            goal_steps = steps;
            let _ = lines.next();
        }
    }
    let goal_steps = goal_steps;

    let map = Map::from_input(lines);
    dprintln!("Map: {:?}", map);

    writeln!(output, "{}", map.find_all_in_dist(goal_steps)).unwrap();
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
    fn sample() {
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
}
