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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum InsideStatus {
    Inside,
    Outside,
    Pipe,
    Unknown,
}

#[derive(Debug)]
struct Node {
    position: Position,
    neighbours: HashSet<Direction>,
    status: Status,
    inside_status: InsideStatus,

    // only valid for pipes
    sides: Vec<HashSet<Direction>>,
    outside_side: Option<usize>,
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
    const LEFT: Direction = Direction::left();
    const RIGHT: Direction = Direction::right();
    const UP: Direction = Direction::up();
    const DOWN: Direction = Direction::down();

    const fn from(x: i64, y: i64) -> Direction { Direction {x, y}}

    const fn up() -> Direction { Direction::from(0, -1) }
    const fn down() -> Direction { Direction::from(0, 1) }
    const fn left() -> Direction { Direction::from(-1, 0) }
    const fn right() -> Direction { Direction::from(1, 0) }

    fn from_to(from: &Position, to: &Position) -> Direction {
        Direction {
            x: to.x as i64 - from.x as i64,
            y: to.y as i64- from.y as i64,
        }
    }
}

impl Node {
    fn sides_for(c: char) -> Vec<HashSet<Direction>> {
        match c {
            '|' => vec![HashSet::from([Direction::left()]), HashSet::from([Direction::right()])],
            '-' => vec![HashSet::from([Direction::up()]), HashSet::from([Direction::down()])],
            'L' => vec![HashSet::from([Direction::left(), Direction::down()]), HashSet::from([])],
            'J' => vec![HashSet::from([]), HashSet::from([Direction::right(), Direction::down()])],
            '7' => vec![HashSet::from([]), HashSet::from([Direction::up(), Direction::right()])],
            'F' => vec![HashSet::from([Direction::up(), Direction::left()]), HashSet::from([])],
            '.' | 'S'  => vec![HashSet::from([]), HashSet::from([])],
            _ => panic!("Don't know this input!"),
        }
    }

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
            inside_status: InsideStatus::Unknown,

            sides: Node::sides_for(c),
            outside_side: None,
        }
    }

    fn side_in(&self, dir: &Direction) -> Option<InsideStatus> {
        let outside = self.outside_side.unwrap();
        let inside = (outside + 1) % 2;
        if self.sides[outside].contains(dir) {
            return Some(InsideStatus::Outside);
        } else if self.sides[inside].contains(dir) {
            return Some(InsideStatus::Inside);
        }
        return None
    }

    fn mark_outside(&mut self, dir: &Direction) {
        if self.sides[0].contains(dir) {
            self.outside_side = Some(0);
        } else if self.sides[1].contains(dir) {
            self.outside_side = Some(1);
        } else {
            panic!("Tried to mark outside and but it was not there. Node: {:?}, dir: {:?}", self, dir);
        }
    }

    fn mark_inside(&mut self, dir: &Direction) {
        if self.sides[0].contains(dir) {
            self.outside_side = Some(1);
        } else if self.sides[1].contains(dir) {
            self.outside_side = Some(0);
        } else {
            panic!("Tried to mark inside and but it was not there. Node: {:?}, dir: {:?}", self, dir);
        }
    }

    fn mark(&mut self, dir: &Direction, is: InsideStatus) {
        match is {
            InsideStatus::Outside => self.mark_outside(dir),
            InsideStatus::Inside => self.mark_inside(dir),
            _ => panic!("Trying to mark unknown"),
        }
    }

    fn mark_inverted(&mut self, dir: &Direction, is: InsideStatus) {
        match is {
            InsideStatus::Outside => self.mark_inside(dir),
            InsideStatus::Inside => self.mark_outside(dir),
            _ => panic!("Trying to mark_inverted unknown"),
        }
    }

    fn is_side(&self, dir: &Direction) -> bool {
        self.sides[0].contains(dir) || self.sides[1].contains(dir)
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

    fn outside_adjacents(&self, at: &Position) -> Vec<Position> {
        let node = self.node_at(at);
        let outside = match node.outside_side {
            Some(o) => o,
            _ => panic!("outside not marked for: {:?}", node),
        };

        return node.sides[outside].iter().filter(|d| self.is_valid_move(at, &d)).
            map(|d| at.move_in(&d)).collect();
    }

    fn inside_adjacents(&self, at: &Position) -> Vec<Position> {
        let node = self.node_at(at);
        let outside = match node.outside_side {
            Some(o) => o,
            _ => panic!("inside not marked for: {:?}", node),
        };
        let inside = (outside + 1) % 2;

        return node.sides[inside].iter().filter(|d| self.is_valid_move(at, &d)).
            map(|d| at.move_in(&d)).collect();
    }

    fn pipe_adjacents(&self, at: &Position) -> Vec<Position> {
        let mut pipes = vec![];
        let node = self.node_at(at);
        for d in vec![Direction::up(), Direction::left(), Direction::down(), Direction::right()] {
            if node.sides[0].contains(&d) || node.sides[0].contains(&d) {
                continue
            }
            if self.is_valid_move(at, &d) {
                pipes.push(at.move_in(&d));
            }
        }
        pipes
    }

    fn neighbours_from(&self, pos: &Position) -> Vec<Position> {
        if pos.y >= self.nodes.len() || pos.y >= self.nodes[0].len() {
            panic!("Trying to access non-existing position")
        }

        let mut neighs = vec![];
        for d in &self.node_at(pos).neighbours {
            if self.is_valid_move(pos, d) {
                neighs.push(pos.move_in(&d));
            }
        }
        neighs
    }

    fn adjacents(&self, pos: &Position) -> Vec<Position> {
        if pos.y >= self.nodes.len() || pos.y >= self.nodes[0].len() {
            panic!("Trying to access non-existing position")
        }
        let mut adjs = vec![];
        for d in vec![Direction::up(), Direction::left(), Direction::down(), Direction::right()] {
            if self.is_valid_move(pos, &d) {
                adjs.push(pos.move_in(&d));
            }
        }
        adjs
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

        let start = self.node_at_mut(&start_pos);
        if HashSet::from([Direction::up(), Direction::down()]) == dirs {
            start.sides = Node::sides_for('|');
        }
        if HashSet::from([Direction::left(), Direction::right()]) == dirs {
            start.sides = Node::sides_for('-');
        }
        if HashSet::from([Direction::up(), Direction::right()]) == dirs {
            start.sides = Node::sides_for('L');
        }
        if HashSet::from([Direction::up(), Direction::left()]) == dirs {
            start.sides = Node::sides_for('J');
        }
        if HashSet::from([Direction::left(), Direction::down()]) == dirs {
            start.sides = Node::sides_for('7');
        }
        if HashSet::from([Direction::right(), Direction::down()]) == dirs {
            start.sides = Node::sides_for('F');
        }

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

    fn parameter_positions(&self) -> Vec<(Direction, Position)> {
        let max_x = self.nodes[0].len() - 1;
        let max_y = self.nodes.len() - 1;

        let horizontal_top = (0..self.nodes[0].len()).map(|x| Position::from(x, 0)).
            map(|p| (Direction::up(), p));
        let horizontal_bottom = (0..self.nodes[0].len()).map(|x| Position::from(x, max_y)).
            map(|p| (Direction::down(), p));
        let vertical_left = (0..self.nodes.len()).map(|y| Position::from(0, y)).
            map(|p| (Direction::left(), p));
        let vertical_right= (0..self.nodes.len()).map(|y| Position::from(max_x, y)).
            map(|p| (Direction::right(), p));

        horizontal_top.chain(horizontal_bottom).chain(vertical_left).chain(vertical_right).collect()
    }

    fn propagate_outsides(&mut self, from: &Position, to: &Position) {
        let dir = Direction::from_to(from, to);

        let mut propagated = false;

        // Horizontal
        if dir.y == 0 {
            if let Some(side) = self.node_at(from).side_in(&Direction::UP) {
                let node_to = self.node_at_mut(to);
                if node_to.is_side(&Direction::UP) {
                    node_to.mark(&Direction::UP, side);
                    propagated = true;
                } else {
                    node_to.mark_inverted(&Direction::DOWN, side);
                    propagated = true;
                }
            }
            if !propagated {
                if let Some(side) = self.node_at(from).side_in(&Direction::DOWN) {
                    let node_to = self.node_at_mut(to);
                    if node_to.is_side(&Direction::DOWN) {
                        self.node_at_mut(to).mark(&Direction::DOWN, side);
                        propagated = true;
                    } else {
                        node_to.mark_inverted(&Direction::UP, side);
                        propagated = true;
                    }
                }
            }
        } else {
            if let Some(side) = self.node_at(from).side_in(&Direction::LEFT) {
                let node_to = self.node_at_mut(to);
                if node_to.is_side(&Direction::LEFT) {
                    self.node_at_mut(to).mark(&Direction::LEFT, side);
                    propagated = true;
                } else {
                    node_to.mark_inverted(&Direction::RIGHT, side);
                    propagated = true;
                }
            } 
            if !propagated {
                if let Some(side) = self.node_at(from).side_in(&Direction::RIGHT) {
                    let node_to = self.node_at_mut(to);
                    if node_to.is_side(&Direction::RIGHT) {
                        self.node_at_mut(to).mark(&Direction::RIGHT, side);
                        propagated = true;
                    } else {
                        node_to.mark_inverted(&Direction::LEFT, side);
                        propagated = true;
                    }
                }
            }
        }
        if !propagated {
            panic!("Failed to propagate from {:?} to {:?}", from, to);
        }
    }

    fn find_insides(&mut self) -> i64 {
        let mut insides = 0;

        let mut outside_queue = VecDeque::new();
        let mut inside_queue = VecDeque::new();

        for (dir, pos) in self.parameter_positions() {
            let node = self.node_at_mut(&pos);
            match node.status {
                Status::Start | Status::Distance(_) => {
                    node.inside_status = InsideStatus::Pipe;
                    node.mark_outside(&dir);
                    inside_queue.push_back(node.position.clone());
                }
                Status::Empty | Status::Unvisited => {
                    node.inside_status = InsideStatus::Outside;
                    outside_queue.push_back(node.position.clone());
                },
            }
        }

        while let Some(pos) = outside_queue.pop_front() {
            for adj_pos in self.adjacents(&pos) {
                let anode = self.node_at_mut(&adj_pos);
                if anode.inside_status != InsideStatus::Unknown {
                    continue
                }
                match anode.status {
                    Status::Start | Status::Distance(_) => {
                        anode.inside_status = InsideStatus::Pipe;
                        anode.mark_outside(&Direction::from_to(&anode.position, &pos));
                        inside_queue.push_back(anode.position.clone());
                    }
                    Status::Empty | Status::Unvisited => {
                        anode.inside_status = InsideStatus::Outside;
                        outside_queue.push_back(anode.position.clone());
                    },
                }
            }
        }

        fn handle_adjacent(graph: &mut Graph, from: &Position, pos: &Position, s: InsideStatus,
                queue: &mut VecDeque<Position>, insides: &mut i64) {
            let anode = graph.node_at_mut(pos);
            if anode.inside_status != InsideStatus::Unknown {
                return
            }
            match anode.status {
                Status::Start | Status::Distance(_) => {
                    anode.inside_status = InsideStatus::Pipe;
                    let dir_back = Direction::from_to(pos, from);
                    anode.mark(&dir_back, s);
                },
                Status::Empty | Status::Unvisited => {
                    anode.inside_status = s;
                    if s == InsideStatus::Inside {
                        *insides += 1;
                    }
                },
            }
            queue.push_back(anode.position.clone());
        }

        while let Some(pos) = inside_queue.pop_front() {
            let node = self.node_at(&pos);
            match node.inside_status {
                InsideStatus::Pipe => {
                    for o in self.outside_adjacents(&pos) {
                        handle_adjacent(
                            self, &pos, &o, InsideStatus::Outside, &mut inside_queue, &mut insides);
                    }
                    for i in self.inside_adjacents(&pos) {
                        handle_adjacent(
                            self, &pos, &i, InsideStatus::Inside, &mut inside_queue, &mut insides);
                    }
                    for p in self.pipe_adjacents(&pos) {
                        {
                            let pnode = self.node_at_mut(&p);
                            if pnode.inside_status != InsideStatus::Unknown {
                                continue
                            }
                            pnode.inside_status = InsideStatus::Pipe;
                            inside_queue.push_back(p);
                        }
                        self.propagate_outsides(&pos, &p);
                    }
                },
                InsideStatus::Unknown => {
                    panic!("There should be no unknowns here. Node: {:?}", node)
                },
                s => {
                    for adj_pos in self.adjacents(&pos) {
                        handle_adjacent(self, &pos, &adj_pos, s, &mut inside_queue, &mut insides);
                    }
                },
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

    writeln!(output, "{}", graph.find_insides()).unwrap();
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
