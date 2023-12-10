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

    fn is_vertical(&self) -> bool {
         self.neighbours == HashSet::from([ Direction::up(), Direction::down() ])
    }

    fn is_half_vertical(&self) -> Option<Direction> {
        if self.neighbours.is_empty() ||
             self.neighbours == HashSet::from([ Direction::left(), Direction::right() ]) ||
             self.neighbours == HashSet::from([ Direction::up(), Direction::down() ]) {
            return None;
        }

        if self.neighbours.contains(&Direction::down()) {
            return Some(Direction::down());
        }
        if self.neighbours.contains(&Direction::up()) {
            return Some(Direction::up());
        }
        return None;
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

    fn start_neighbours(&mut self) -> Vec<Position> {
        let start_pos = self.start.clone();
        let mut neighs = vec![];
        let mut dirs = HashSet::new();
        for d in vec![Direction::up(), Direction::left(), Direction::down(), Direction::right()] {
            if !self.is_valid_move(&start_pos, &d) { continue }
            let maybe_neigh = self.node_at(&start_pos.move_in(&d));
            for pos in self.neighbours_from(&maybe_neigh.position) {
                if pos == start_pos {
                    neighs.push(maybe_neigh.position.clone());
                    dirs.insert(d);
                    break
                }
            }
        }
        self.node_at_mut(&start_pos).neighbours = dirs;
        neighs
    }

    fn furthest_on_loop(&mut self) -> i64 {
        let mut max_dist = 0;
        let mut queue = VecDeque::new();

        let start_neighbours = self.start_neighbours();
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

    fn count_insides(&self) -> i64 {
        let mut insides = 0;
        for row in &self.nodes {
            let mut num_vert = 0;
            let mut open_half = None;
            for n in row {
                match n.status {
                    Status::Start | Status::Distance(_) => {
                        if n.is_vertical() {
                            num_vert += 1;
                        } else if let Some(dir) = n.is_half_vertical() {
                            open_half = match open_half {
                                Some(hdir) => {
                                    if hdir != dir {
                                        num_vert += 1;
                                    }
                                    None
                                },
                                None => Some(dir),
                            }
                        }
                    },
                    _ => {
                        if num_vert % 2 != 0 {
                            insides += 1;
                        }
                    },
                }
            }
        }
        insides
    }
}


fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut graph = Graph::from_lines(lines);
    dprintln!("Graph: {:?}", graph);
    let _ = graph.furthest_on_loop();

    writeln!(output, "{}", graph.count_insides()).unwrap();
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
            "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........",
            "4",
        );
    }

    #[test]
    fn sample2() {
        test_ignore_whitespaces(
            "..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........",
            "4",
        );
    }

    #[test]
    fn sample3() {
        test_ignore_whitespaces(
            ".F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...",
            "8",
        );
    }

    #[test]
    fn sample4() {
        test_ignore_whitespaces(
            "FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L",
            "10",
        );
    }

    #[test]
    fn mine() {
        test_ignore_whitespaces(
            "S-----7
            |.F-7.|
            |.|.|.|
            |.|FJ.|
            |.||..|
            |.|L--J
            L-J....",
            "10",
        );
    }
}
