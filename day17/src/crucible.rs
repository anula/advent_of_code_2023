//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
struct XY {
    x: i64,
    y: i64,
}

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

impl XY {
    const AROUND: [XY; 4] = [
        XY::new(0, -1),
        XY::new(1, 0),
        XY::new(0, 1),
        XY::new(-1, 0),
    ];
    const fn new(x: i64, y: i64) -> XY { XY {x, y} }

    const fn add(&self, other: &XY) -> XY { XY { x: self.x + other.x, y: self.y + other.y } }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Grid {
    nodes: Vec<Vec<i64>>,
}


#[derive(Debug, PartialEq, Eq, Hash)]
struct State(i64, XY, usize, i64);
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
           .then_with(|| self.1.cmp(&other.1))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn from_input<I>(lines: I) -> Grid
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for c in line.chars() {
                nodes[y].push(c.to_string().parse().unwrap());
            }
        }

        Grid {
            nodes,
        }
    }

    fn is_valid(&self, pos: &XY) -> bool {
        pos.x >= 0 && pos.y >= 0 &&
                pos.x < self.nodes[0].len() as i64 && pos.y < self.nodes.len() as i64
    }

    fn node_at(&self, at: &XY) -> i64 {
        if !self.is_valid(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        self.nodes[at.y as usize][at.x as usize]
    }

    fn neighbours(&self, at: &XY, dir: usize, len: i64) -> Vec<(XY, usize, i64)> {
        let mut neighs = Vec::new();
        for (i, diff) in XY::AROUND.iter().enumerate() {
            if (i + 2) % 4 == dir {
                // ignore opposite
                continue;
            }

            if i == dir && len >= 3 {
                continue;
            }
            let potential = at.add(&diff);
            if self.is_valid(&potential) {
                let new_dir = i;
                let new_len = if new_dir == dir { len + 1 } else { 1 };
                neighs.push((potential, new_dir, new_len));
            }
        }
        dprintln!("neighs({:?}, {:?}, {:?}): {:?}", at, dir, len, neighs);
        neighs
    }

    fn height(&self) -> usize { self.nodes.len() }
    fn width(&self) -> usize { self.nodes[0].len() }

    fn debug_print_distances(&self, dists: &HashMap<(XY, i64), Vec<i64>>) {
        for i in 0..4 {
            dprintln!("For i: {}", i);
            for y in 0..self.height() {
                let mut line_str = String::new();
                for x in 0..self.width() {
                    line_str += "(";
                    for len in 0..4 {
                        if let Some(es) = dists.get(&(XY::new(x as i64, y as i64), len)) {
                            if es[i] != i64::MAX {
                                line_str += &format!(" {:03},", es[i]);
                            } else {
                                line_str += " MAX,";
                            }
                        } else {
                            line_str += "   -,";
                        }
                    }
                    line_str += ");";
                }
                dprintln!("{}", line_str);
            }
        }
    }

    fn dijkstra(&self, start: XY, goal: XY) -> i64 {
        let mut distances: HashMap<(XY, i64), Vec<i64>> = HashMap::new();

        let mut heap = BinaryHeap::new();
        heap.push(State(0, start, RIGHT, 0));
        for i in 0..4 {
            distances.insert((start, i as i64), vec![0, 0, 0, 0]);
        }

        while let Some(State(dist, pos, dir, len)) = heap.pop() {
            dprintln!("Got: {:?}, {:?}, {:?}, {}", dist, pos, dir, len);
            if pos == goal {
                self.debug_print_distances(&distances);
                return dist;
            }

            if let Some(es) = distances.get(&(pos, len)) {
                if es[dir] < dist {
                    continue;
                }
            }

            for (n, new_dir, new_len) in self.neighbours(&pos, dir, len) {
                let new_cost = dist + self.node_at(&n);
                if let Some(es) = distances.get_mut(&(n, new_len)) {
                    if new_cost < es[new_dir] {
                        es[new_dir] = new_cost;
                        heap.push(State(new_cost, n, new_dir, new_len));
                    }

                } else {
                    let mut new_dists = vec![i64::MAX; 4];
                    new_dists[new_dir] = new_cost;
                    distances.insert((n, new_len), new_dists);
                    heap.push(State(new_cost, n, new_dir, new_len));
                }
            }
        }

        self.debug_print_distances(&distances);

        panic!("No path found!")
    }
}


fn solve<R: BufRead, W: Write>(input: R, mut output: W) {

    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    dprintln!("reading");
    let grid = Grid::from_input(lines);
    dprintln!("Grid: {:?}", grid);

    let start = XY::new(0, 0);
    let goal = XY::new(grid.height() as i64 - 1, grid.width() as i64 - 1);

    writeln!(output, "{}", grid.dijkstra(start, goal)).unwrap();
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
            "2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533",
            "102",
        );
    }

    #[test]
    fn mine() {
        test_ignore_whitespaces(
            "9199999999999
             9199999999999
             9119999999999
             9919911199999
             9911919199999
             9991119119999
             9999999919999
             9999999119999
             9999999199999
             9999999119999
             9999999919999
             9999999911199
             9999999999111",
            "30",
        );
    }
}
