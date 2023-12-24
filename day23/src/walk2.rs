use std::cmp::max;
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

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Edge {
    fn new(from: XY, to: XY) -> Edge {
        Edge { from, to, cost: -1 }
    }

    fn new_cost(from: XY, to: XY, cost: i64) -> Edge {
        Edge { from, to, cost }
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

#[allow(dead_code)]
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

    fn neighbours(&self, at: XY, from: XY) -> Vec<XY> {
        let mut neighs = Vec::new();

        for d in &Direction::ALL {
            let potential = at.add(&d.as_direction());
            if !self.is_valid(&potential) {
                continue;
            }
            let neigh = self.node_at(&potential);
            if potential == from {
                continue;
            }
            match neigh.typ {
                Type::Forest => continue,
                _ => {},
            }
            neighs.push(potential);
        }
        neighs
    }

    fn compute_simplified_graph(&self) -> Graph {
        let mut graph = HashMap::<XY, Vec<(XY, i64)>>::new();
        let mut visited_edges = HashSet::new();
        let mut queue = VecDeque::new();

        let mut next_start = self.start.clone();
        next_start.y = 1;
        queue.push_back((self.start, next_start));

        while let Some((mut from, mut node)) = queue.pop_front() {
            let mut edge = Edge::new_cost(from, node, 1);

            loop {
                let neigh = self.neighbours(node, from);
                if neigh.len() != 1 {
                    break;
                }

                from = node;
                node = neigh[0];
                edge.cost += 1;
                edge.to = node;
            }
            if visited_edges.contains(&(edge.from, edge.to)) {
                continue;
            }
            visited_edges.insert((edge.from, edge.to));
            visited_edges.insert((edge.to, edge.from));

            graph.entry(edge.from).or_insert(vec![]).push((edge.to, edge.cost));
            graph.entry(edge.to).or_insert(vec![]).push((edge.from, edge.cost));

            for neigh in self.neighbours(node, from) {
                queue.push_back((node, neigh));
            }
        }

        Graph {
            neighs: graph,
            start: self.start,
            end: self.end,

            orig_width: self.width(),
            orig_height: self.height(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    neighs: HashMap<XY, Vec<(XY, i64)>>,
    start: XY,
    end: XY,

    orig_width: usize,
    orig_height: usize,
}

impl Graph {

    fn find_longest_path(&self) -> i64 {
        let (nodes, result) = self.find_longest_path_int(self.start, &mut HashSet::new());
        println!("nodes: {:?}", nodes);
        result
    }

    fn find_longest_path_int(&self, pos: XY, visited: &mut HashSet<XY>) -> (Vec<(XY, i64)>, i64) {
        if pos == self.end {
            return (vec![(pos, 0)], 0);
        }
        let mut nodes_used = Vec::new();
        visited.insert(pos);
        let mut max_len = i64::MIN;
        let mut di = 0;
        for &(n, dist) in self.neighs.get(&pos).unwrap() {
            if visited.contains(&n) {
                continue;
            }
            let (pot_nodes, pot_dist) = self.find_longest_path_int(n, visited);
            let potential = dist + pot_dist;
            if potential > max_len {
                nodes_used = pot_nodes;
                max_len = potential;
                di = dist;
            }
        }
        visited.remove(&pos);

        nodes_used.push((pos, di));


        (nodes_used, max_len)
    }
}




fn solve<R: BufRead, W: Write>(input: R, mut output: W) {

    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let map = Map::from_input(lines);
    let graph = map.compute_simplified_graph();
    println!("\n{:?}", graph);

    writeln!(output, "{}", graph.find_longest_path()).unwrap();
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
            "154",
        );
    }
}
