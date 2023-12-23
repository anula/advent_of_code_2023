use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::HashMap;

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
    const fn sub(&self, other: &XY) -> XY { XY { x: self.x - other.x, y: self.y - other.y } }

    const fn ux(&self) -> usize { self.x as usize }
    const fn uy(&self) -> usize { self.y as usize }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Edge {
    from: XY,
    to: XY,

    cost: i64,
}

impl Edge {
    fn new(from: XY, to: XY) -> Edge {
        Edge { from, to, cost: -1 }
    }

    fn reversed(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            cost: self.cost,
        }
    }

    fn direction(&self) -> Direction {
        Direction::from(self.to.sub(&self.from))
    }
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

    const fn from(xy: XY) -> Direction {
        match xy {
            XY{x: 0, y: -1} => UP,
            XY{x: 1, y: 0}  => RIGHT,
            XY{x: 0, y: 1}  => DOWN,
            XY{x: -1, y: 0} => LEFT,
            _ => panic!("This is no direction"),
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

    const fn as_char(&self) -> char {
        match self {
            UP    => '^',
            RIGHT => '>',
            DOWN  => 'v',
            LEFT  => '<',
        }
    }

    const fn opposite(&self) -> Direction {
        match self {
            UP    => DOWN,
            RIGHT => LEFT,
            DOWN  => UP,
            LEFT  => RIGHT,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Type {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Node {
    typ: Type,
}

impl Node {
    fn from_char(c: char) -> Node {
        let typ = match c {
            '.' => Type::Path,
            '#' => Type::Forest,
            '^' => Type::Slope(UP),
            '>' => Type::Slope(RIGHT),
            'v' => Type::Slope(DOWN),
            '<' => Type::Slope(LEFT),
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
    end: XY,
}

impl Map {
    fn from_input<I>(lines: I) -> Map
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();
        let mut start = XY::new(-1, -1);
        let mut end = XY::new(-1, -1);

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for (x, c) in line.char_indices() {
                nodes[y].push(Node::from_char(c));

                if y == 0 && c == '.' {
                    start = XY::new(x as i64, y as i64);
                }
                if c == '.' {
                    end = XY::new(x as i64, y as i64);
                }
            }
        }

        Map {
            nodes,
            start,
            end,
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

        let node = self.node_at(at);
        for d in &Direction::ALL {
            let potential = at.add(&d.as_direction());
            if let Type::Slope(only_d) = node.typ {
                if *d != only_d {
                    continue;
                }
            }
            if !self.is_valid(&potential) {
                continue;
            }
            let neigh = self.node_at(&potential);
            match neigh.typ {
                Type::Forest => continue,
                Type::Slope(s_dir) => {
                    if d.opposite() == s_dir {
                        continue;
                    }
                },
                _ => {},
            }
            neighs.push(potential);
        }
        neighs
    }

    fn compute_graph(&self) -> Graph {
        let mut edges = HashSet::new();
        let mut already_added = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(self.start.clone());

        while let Some(node) = queue.pop_front() {
            if already_added.contains(&node) {
                continue;
            }

            for neigh in self.neighbours(&node) {
                let potential_edge = Edge::new(node, neigh);
                if edges.contains(&potential_edge.reversed()) {
                    continue;
                }

                edges.insert(potential_edge);
                queue.push_back(neigh);
            }

            already_added.insert(node);
        }

        Graph {
            edges: edges.into_iter().collect(),
            nodes_num: already_added.len(),

            orig_width: self.width(),
            orig_height: self.height(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Graph {
    edges: Vec<Edge>,
    nodes_num: usize,

    orig_width: usize,
    orig_height: usize,
}

impl Graph {

    fn find_shortest_path(&self, start: XY, end: XY) -> i64 {
        let mut dist = HashMap::new();
        dist.insert(start, 0);

        for step in 0..=self.nodes_num {
            let mut changes = false;
            for edge in &self.edges {
                let dist_from = if let Some(d) = dist.get(&edge.from) {
                    d
                } else {
                    continue;
                };
                let curr_dist_to = if let Some(d) = dist.get(&edge.to) {
                    *d
                } else {
                    0
                };
                let new_dist = dist_from + edge.cost;

                if new_dist < curr_dist_to {
                    if step == self.nodes_num {
                        panic!("Negative cycle!");
                    }
                    changes = true;
                    dist.insert(edge.to, new_dist);
                }
            }

            if !changes {
                break;
            }
        }


        *dist.get(&end).unwrap()
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut chars = vec![vec!['#'; self.orig_width]; self.orig_height];
        for edge in &self.edges {
            let dir = edge.direction().as_char();
            chars[edge.from.uy()][edge.from.ux()] = dir;

            if chars[edge.to.uy()][edge.to.ux()] == '#' {
                chars[edge.to.uy()][edge.to.ux()] = '.'
            }
        }

        for y in 0..self.orig_height {
            println!("{}", chars[y].iter().collect::<String>());
        }
    }
}




fn solve<R: BufRead, W: Write>(input: R, mut output: W) {

    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let map = Map::from_input(lines);
    dprintln!("Map: {:?}", map);
    let graph = map.compute_graph();
    dprintln!("edges: {:?}", graph);

    writeln!(output, "{}", -graph.find_shortest_path(map.start, map.end)).unwrap();
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
            "#.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#",
            "94",
        );
    }
}
