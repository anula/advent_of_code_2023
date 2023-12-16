//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::VecDeque;

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
    const fn as_entry(&self) -> usize {
        match self {
            UP => 0,
            RIGHT => 1,
            DOWN => 2,
            LEFT => 3,
        }
    }

    const fn as_direction(&self) -> XY {
        match self {
            UP => XY::new(0, -1),
            RIGHT => XY::new(1, 0),
            DOWN => XY::new(0, 1),
            LEFT => XY::new(-1, 0),
        }
    }

    const fn opposite(&self) -> Direction {
        match self {
            UP => DOWN,
            RIGHT => LEFT,
            DOWN => UP,
            LEFT => RIGHT,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum State {
    Unvisited,
    Energized,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Type {
    Empty,
    Mirror,
    BackMirror, 
    SplitHor,
    SplitVert,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    routes: Vec<Vec<Direction>>,
    states: Vec<State>,

    typ: Type,
}


impl Node {
    fn from_char(c: char) -> Node {
        let typ = match c {
            '.' => Type::Empty,
            '/' => Type::Mirror,
            '\\' => Type::BackMirror,
            '|' => Type::SplitVert,
            '-' => Type::SplitHor,
            _ => panic!("Wrong char!"),
        };
        let routes = match typ {
            Type::Empty => vec![
                vec![DOWN],
                vec![LEFT],
                vec![UP],
                vec![RIGHT],
            ],
            Type::Mirror => vec![
                vec![LEFT],
                vec![DOWN],
                vec![RIGHT],
                vec![UP],
            ],
            Type::BackMirror => vec![
                vec![RIGHT],
                vec![UP],
                vec![LEFT],
                vec![DOWN],
            ],
            Type::SplitVert => vec![
                vec![DOWN],
                vec![UP, DOWN],
                vec![UP],
                vec![UP, DOWN],
            ],
            Type::SplitHor => vec![
                vec![LEFT, RIGHT],
                vec![LEFT],
                vec![LEFT, RIGHT],
                vec![RIGHT],
            ],
        };
        let states = vec![State::Unvisited; 4];
        Node {
            routes,
            states,
            typ,
        }
    }

    fn is_energized(&self) -> bool {
        self.states.iter().any(|&x| x == State::Energized)
    }

    fn mark_energized(&mut self, from: Direction) {
        let from_idx = from.as_entry();
        self.states[from_idx] = State::Energized;

        for d in &self.routes[from_idx] {
            self.states[d.as_entry()] = State::Energized;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Map {
    nodes: Vec<Vec<Node>>,
}

impl Map {
    fn from_input<I>(lines: I) -> Map
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for c in line.chars() {
                nodes[y].push(Node::from_char(c));
            }
        }

        Map {
            nodes,
        }
    }

    fn energized(&self) -> i64 {
        self.nodes.iter().map(|ns|
            ns.iter().map(|n| if n.is_energized() {1} else {0}).sum::<i64>()
        ).sum()
    }

    fn node_at(&self, at: &XY) -> &Node {
        if !self.is_valid(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        &self.nodes[at.y as usize][at.x as usize ]
    }

    fn node_at_mut(&mut self, at: &XY) -> &mut Node {
        if !self.is_valid(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        &mut self.nodes[at.y as usize][at.x as usize ]
    }

    fn is_valid(&self, pos: &XY) -> bool {
        pos.x >= 0 && pos.y >= 0 &&
                pos.x < self.nodes[0].len() as i64 && pos.y < self.nodes.len() as i64
    }

    fn neighbours(&self, from: &Direction, at: &XY) -> Vec<(Direction, XY)> {
        let mut neighs = Vec::new();
        let node = self.node_at(at);
        for d in &node.routes[from.as_entry()] {
            //dprintln!("d: {:?}", d);
            let potential = at.add(&d.as_direction());
            //dprintln!("potential: {:?}", potential);
            if self.is_valid(&potential) {
                neighs.push((d.opposite(), potential));
            }
        }
        neighs
    }

    fn bfs(&mut self, from_dir: Direction, start_position: XY) {
        let mut queue = VecDeque::<(Direction, XY)>::new();

        queue.push_back((from_dir, start_position));

        while let Some((entry, pos)) = queue.pop_front() {
            dprintln!("entering: ({:?}, {:?})", entry, pos);
            {
                let node = self.node_at_mut(&pos);
                dprintln!("node: {:?}", node);
                if node.states[entry.as_entry()] != State::Unvisited {
                    continue;
                }

                node.mark_energized(entry);
                dprintln!("now energized: {:?}", node);
            }

            for n in self.neighbours(&entry, &pos) {
                queue.push_back(n);
            }
        }
    }
}


fn solve<R: BufRead, W: Write>(input: R, mut output: W) {

    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut map = Map::from_input(lines);
    dprintln!("Map: {:?}", map);
    map.bfs(LEFT, XY::new(0, 0));

    writeln!(output, "{}", map.energized()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exact(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        assert_eq!(String::from_utf8(actual_out).unwrap(), output);
    }

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
            ".|...\\....
            |.-.\\.....
            .....|-...
            ........|.
            ..........
            .........\\
            ..../.\\\\..
            .-.-/..|..
            .|....-|.\\
            ..//.|....",
            "46",
        );
    }
}
