use std::cmp::max;
use std::io::{BufRead, BufReader, Write};
use std::collections::{HashSet, VecDeque};

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Direction {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Status {
    Start,
    Empty,
    Unvisited,
    Distance(i64),
}

#[derive(Debug)]
struct Node {
    position: Position,
    neighbours: HashSet<Direction>,
    status: Status,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Node>>,
    start: Position,
}

impl Position {
    fn from(x: usize, y: usize) -> Position { Position {x, y}}
    fn move_in(&self, d: &Direction) -> Position { Position {
        x: (self.x as i64 + d.x) as usize,
        y: (self.y as i64 + d.y) as usize,
    } }
}

impl Direction {
    fn from(x: i64, y: i64) -> Direction { Direction {x, y}}

    fn up() -> Direction { Direction::from(0, -1) }
    fn down() -> Direction { Direction::from(0, 1) }
    fn left() -> Direction { Direction::from(-1, 0) }
    fn right() -> Direction { Direction::from(1, 0) }
}

impl Node {
    fn from_char(c: char, pos: &Position) -> Node {
        Node {
            position: pos.clone(),
            neighbours: match c {
                '|' => HashSet::from([ Direction::up(), Direction::down() ]),
                '-' => HashSet::from([ Direction::left(), Direction::right() ]),
                'L' => HashSet::from([ Direction::up(), Direction::right() ]),
                'J' => HashSet::from([ Direction::up(), Direction::left() ]),
                '7' => HashSet::from([ Direction::down(), Direction::left() ]),
                'F' => HashSet::from([ Direction::down(), Direction::right() ]),
                '.' => HashSet::new(),
                'S' => HashSet::new(),
                _ => panic!("Don't know this input!"),
            },
            status: match c {
                '|' | '-' | 'L' | 'J' | '7' | 'F' => Status::Unvisited,
                '.' => Status::Empty,
                'S' => Status::Start,
                _ => panic!("Don't know this input!"),
            },
        }
    }
}

impl Graph {
    fn from_lines<I>(lines: I) -> Graph
        where I: Iterator<Item = String>,
    {
        let mut nodes = Vec::new();
        let mut start = Position::from(usize::MAX, usize::MAX);
        for (y, line) in lines.enumerate() {
            nodes.push(Vec::new());
            for (x, c) in line.trim().char_indices() {
                let pos = Position::from(x, y);
                let node = Node::from_char(c, &pos);
                if node.status == Status::Start {
                    start = pos.clone();
                }
                nodes[y].push(node);
            }
        }

        Graph {
            nodes: nodes,
            start: start,
        }
    }

    fn node_at(&self, pos: &Position) -> &Node {
        &self.nodes[pos.y][pos.x]
    }

    fn node_at_mut(&mut self, pos: &Position) -> &mut Node {
        &mut self.nodes[pos.y][pos.x]
    }

    fn neighbours_from(&self, pos: &Position) -> Vec<Position> {
        if pos.y >= self.nodes.len() || pos.y >= self.nodes[0].len() {
            panic!("Trying to access non-existing position")
        }

        let mut neighs = vec![];
        for d in &self.node_at(pos).neighbours {
            if !self.is_valid_move(pos, d) {
                continue
            }
            neighs.push(pos.move_in(&d));
        }
        neighs
    }

    fn is_valid_move(&self, pos: &Position, dir: &Direction) -> bool {
        let new_x: i64 = pos.x as i64 + dir.x;
        let new_y: i64 = pos.y as i64 + dir.y;
        if new_x < 0 || new_y < 0 { 
            return false;
        }
        if new_y as usize >= self.nodes.len() || new_x as usize >= self.nodes[0].len() {
            return false;
        }
        return true;
    }

    fn start_neighbours(&self) -> Vec<Position> {
        let start_pos = self.start.clone();
        let mut neighs = vec![];
        for d in vec![Direction::up(), Direction::left(), Direction::down(), Direction::right()] {
            if !self.is_valid_move(&start_pos, &d) { continue }
            let maybe_neigh = self.node_at(&start_pos.move_in(&d));
            for pos in self.neighbours_from(&maybe_neigh.position) {
                if pos == start_pos {
                    neighs.push(maybe_neigh.position.clone());
                    break
                }
            }
        }
        neighs
    }

    fn furthest_on_loop(&mut self) -> i64 {
        let mut max_dist = 0;
        let mut queue = VecDeque::new();

        let start_neighbours = self.start_neighbours();
        dprintln!("start neighbours: {:?}", start_neighbours);
        for p in &start_neighbours {
            max_dist = 1;
            let n = self.node_at_mut(p);
            n.status = Status::Distance(1);
            queue.push_back(p.clone());
        }

        while let Some(pos) = queue.pop_front() {
            let curr_dist = if let Status::Distance(dist) = self.node_at(&pos).status {
                dist
            } else {
                panic!("Node without distance was on queue: {:?}", self.node_at(&pos))
            };
            for npos in self.neighbours_from(&pos) {
                let nnode = self.node_at_mut(&npos);
                if nnode.status != Status::Unvisited {
                    continue
                }
                let new_dist = curr_dist + 1;
                nnode.status = Status::Distance(new_dist);
                max_dist = max(max_dist, new_dist);
                queue.push_back(nnode.position.clone());
            }
        }

        max_dist
    }
}


fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut graph = Graph::from_lines(lines);
    dprintln!("Graph: {:?}", graph);

    writeln!(output, "{}", graph.furthest_on_loop()).unwrap();
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
            ".....
            .S-7.
            .|.|.
            .L-J.
            .....",
            "4",
        );
        test_ignore_whitespaces(
            "-L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF",
            "4",
        );
    }

    #[test]
    fn sample2() {
        test_ignore_whitespaces(
            "..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...",
            "8",
        );
        test_ignore_whitespaces(
            "7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ",
            "8",
        );
    }
}
