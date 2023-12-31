use std::cmp::max;
use std::io::{BufRead, BufReader, Write};
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
    const fn newu(x: usize, y: usize) -> XY { XY {x: x as i64, y: y as i64} }

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

    fn bfs(&self, starts: &[XY]) -> Vec<Vec<i64>> {
        let mut dists = vec![vec![i64::MAX; self.width()]; self.height()];

        let mut queue = VecDeque::new();

        for s in starts {
            queue.push_back((0, s.clone()));
            dists[s.y as usize][s.x as usize] = 0;
        }

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

fn multi_count_end_positions(dists: &HashMap<XY, i64>, mod_2: i64) -> i64 {
    let mut count = 0;
    for (_, &dist) in dists {
        if dist % 2 == mod_2 {
            count += 1;
        }
    }

    count
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

#[allow(dead_code)]
fn multi_print_dists(dists: &HashMap<XY, i64>, size: i64, min: i64, max: i64, mod_2: i64) {
    for y in min..=max {
        for x in min..=max {
            let sep = if (x + 1) % size == 0 {
                "|"
            } else {
                " "
            };
            if let Some(d) = dists.get(&XY::new(x, y)) {
                if d % 2 == mod_2 {
                    print!("{: >2}*{}", d, sep);
                } else {
                    print!("{: >3}{}", d, sep);
                }
            } else {
                print!("  #{}", sep);
            }
        }
        if (y + 1) % size == 0 {
            println!();
        }
        println!();
    }
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

fn solve_special_case(goal_steps: i64, map: &Map) -> i64 {
    let init_distances = map.bfs(&vec![map.start]);

    let len = map.width();
    let mid = len / 2;
    dprintln!("len: {}, mid: {}", len, mid);

    let l_mid_point = XY::newu(0, mid);
    let r_mid_point = XY::newu(len - 1, mid);
    let u_mid_point = XY::newu(mid, 0);
    let d_mid_point = XY::newu(mid, len - 1);

    let lu_corner_point = XY::newu(0, 0);
    let ru_corner_point = XY::newu(len - 1, 0);
    let ld_corner_point = XY::newu(0, len - 1);
    let rd_corner_point = XY::newu(len - 1, len - 1);

    let same_mod_2 = goal_steps % 2;
    let inv_mod_2 = (same_mod_2 + 1) % 2;

    let init_out = init_distances[l_mid_point.y as usize][l_mid_point.x as usize];

    let remaining_steps = max(0, goal_steps - init_out);
    let full_tiles_arm = max(0, remaining_steps / len as i64 - 1);
    let has_edge = if init_out >= goal_steps { 0 } else { 1 };
    let internal_edges = if has_edge == 1 { max(0, full_tiles_arm) } else { 0 };
    let external_edges = if has_edge == 1 { internal_edges + 1 } else { 0 };


    let even_tiles = if full_tiles_arm % 2 == 1 {
        max(0, (full_tiles_arm) * (full_tiles_arm) - 1)
    } else {
        max(0, max(0, full_tiles_arm + 1) * max(0, full_tiles_arm + 1) - 1)
    };
    let odd_tiles = if full_tiles_arm % 2 == 1 {
        max(0, max(0, full_tiles_arm + 1) * max(0, full_tiles_arm + 1))
    } else {
        max(0, (full_tiles_arm) * (full_tiles_arm))
    };

    //let odd_in_axis = (full_tiles_arm / 2) + (full_tiles_arm % 2);
    //let even_in_axis = (full_tiles_arm / 2);

    println!("full_tiles_arm: {}", full_tiles_arm);
    println!("internal_edges: {}", internal_edges);
    println!("external_edges: {}", external_edges);
    println!("even_tiles: {}, odd_tiles: {}", even_tiles, odd_tiles);
    println!("has_edge: {}", has_edge);

    let left_dists =  map.bfs(&vec![r_mid_point]);
    let right_dists = map.bfs(&vec![l_mid_point]);
    let up_dists =    map.bfs(&vec![d_mid_point]);
    let down_dists =  map.bfs(&vec![u_mid_point]);

    let lu_dists = map.bfs(&vec![rd_corner_point]);
    let ru_dists = map.bfs(&vec![ld_corner_point]);
    let ld_dists = map.bfs(&vec![ru_corner_point]);
    let rd_dists = map.bfs(&vec![lu_corner_point]);

    let count_init_tile = count_end_positions_vec(&init_distances, same_mod_2, goal_steps + 1);

    let count_even_tile = count_end_positions_vec(&left_dists, inv_mod_2, goal_steps + 1);
    let count_odd_tile = count_end_positions_vec(&left_dists, same_mod_2, goal_steps + 1);

    let dist_to_edge = full_tiles_arm * len as i64 + mid as i64 + 1;
    let mod_for_edge = (goal_steps - dist_to_edge) % 2;
    println!("mod for edge: {}", mod_for_edge);
    println!("left dists:");
    print_dists(&left_dists);
    let count_left_edge =  count_end_positions_vec(&left_dists, mod_for_edge, len as i64);
    let count_right_edge = count_end_positions_vec(&right_dists, mod_for_edge, len as i64);
    let count_up_edge =    count_end_positions_vec(&up_dists, mod_for_edge, len as i64);
    let count_down_edge =  count_end_positions_vec(&down_dists, mod_for_edge, len as i64);

    let internal_edges_starting_corner = full_tiles_arm * len as i64 + 1;
    println!("internal_edges_starting_corner: {}", internal_edges_starting_corner);
    let steps_left_ie = goal_steps - internal_edges_starting_corner;
    let intern_mod_2 = steps_left_ie % 2;

    let count_lu_internal_edge = count_end_positions_vec(&lu_dists, intern_mod_2, steps_left_ie + 1);
    let count_ru_internal_edge = count_end_positions_vec(&ru_dists, intern_mod_2, steps_left_ie + 1);
    let count_ld_internal_edge = count_end_positions_vec(&ld_dists, intern_mod_2, steps_left_ie + 1);
    let count_rd_internal_edge = count_end_positions_vec(&rd_dists, intern_mod_2, steps_left_ie + 1);

    let external_edges_starting_corner = (full_tiles_arm + 1) * len as i64 + 1;
    println!("external_edges_starting_corner: {}", external_edges_starting_corner);
    let steps_left_ee = goal_steps - external_edges_starting_corner;
    let extern_mod_2 = steps_left_ee % 2;
    println!("extern_mod_2: {}", extern_mod_2);
    let count_lu_external_edge = count_end_positions_vec(&lu_dists, extern_mod_2, steps_left_ee + 1);
    let count_ru_external_edge = count_end_positions_vec(&ru_dists, extern_mod_2, steps_left_ee + 1);
    let count_ld_external_edge = count_end_positions_vec(&ld_dists, extern_mod_2, steps_left_ee + 1);
    let count_rd_external_edge = count_end_positions_vec(&rd_dists, extern_mod_2, steps_left_ee + 1);

    println!("count_init_tile: {}", count_init_tile);
    println!("count_even_tile: {}", count_even_tile);
    println!("count_odd_tile: {}", count_odd_tile);

    println!("left edge: {}", count_left_edge);
    println!("right edge: {}", count_right_edge);
    println!("up edge: {}", count_up_edge);
    println!("down edge: {}", count_down_edge);

    println!("---");
    println!("count ru internal: {}", count_ru_internal_edge);
    println!("count ru external: {}", count_ru_external_edge);

    println!("---");
    println!("count lu internal: {}", count_lu_internal_edge);
    println!("count lu external: {}", count_lu_external_edge);

    println!("---");
    println!("count ld internal: {}", count_ld_internal_edge);
    println!("count ld external: {}", count_ld_external_edge);

    println!("---");
    println!("count rd internal: {}", count_rd_internal_edge);
    println!("count rd external: {}", count_rd_external_edge);
    println!("---");

    let result = count_init_tile +
        even_tiles * count_even_tile +
        odd_tiles * count_odd_tile +
        (count_left_edge + count_right_edge + count_up_edge + count_down_edge) * has_edge +
        count_lu_internal_edge * internal_edges +
        count_ru_internal_edge * internal_edges +
        count_ld_internal_edge * internal_edges +
        count_rd_internal_edge * internal_edges +
        count_lu_external_edge * external_edges +
        count_ru_external_edge * external_edges +
        count_ld_external_edge * external_edges +
        count_rd_external_edge * external_edges +
        0;

    result
}

fn parse_input<R: BufRead>(input: R) -> (i64, Vec<String>)
{
    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap()).peekable();

    let mut goal_steps = 26501365;
    if let Some(p) = lines.peek() {
        if let Ok(steps) = p.trim().parse::<i64>() {
            goal_steps = steps;
            let _ = lines.next();
        }
    }
    let goal_steps = goal_steps;

    (goal_steps, lines.collect())
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let (goal_steps, lines) = parse_input(input);

    let map = Map::from_input(lines.into_iter());

    //let brut_dists = map.multi_bfs(goal_steps);
    //multi_print_dists(&brut_dists, map.width() as i64, -17, 25, goal_steps % 2);
    writeln!(output, "{}", solve_special_case(goal_steps, &map)).unwrap();
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

    fn compare_test(goal_steps: i64, lines: &Vec<String>) {
        let map = Map::from_input(lines.clone().into_iter());

        let tester = solve_special_case(goal_steps, &map);

        let brut_dists = map.multi_bfs(goal_steps);
        multi_print_dists(&brut_dists, map.width() as i64, -17, 25, goal_steps % 2);
        let brut = multi_count_end_positions(&brut_dists, goal_steps % 2);

        assert_eq!(tester, brut);
    }

    fn random_input(size: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mid = size / 2;
        for y in 0..size {
            let mut line = String::new();
            for x in 0..size {
                if x == mid && y == mid {
                    line.push('S');
                    continue;
                }
                if x == 0 || y == 0 || x == size - 1 || y == size - 1 || x == mid || y == mid {
                    line.push('.');
                    continue;
                }
                if rand::random() {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            lines.push(line);
        }

        lines
    }

    #[test]
    fn random_7() {
        let step_sizes = vec![10, 17, 24, 31];
        let len = 7;

        let inp = random_input(len);
        for steps in step_sizes {
            println!("{}", steps);
            println!("{}\n", inp.join("\n"));

            compare_test(steps, &inp);
        }
    }

    #[test]
    fn just_init() {
        test_ignore_whitespaces(
            "1
            .....
            .....
            ..S..
            .....
            .....",
            "4",
        );
        test_ignore_whitespaces(
            "1
            .....
            ..#..
            ..S..
            .....
            .....",
            "3",
        );
        test_ignore_whitespaces(
            "2
            .....
            .....
            ..S..
            .....
            .....",
            "9",
        );
    }

    #[test]
    fn init_and_edge() {
        test_ignore_whitespaces(
            "4
            ...
            .S.
            ...",
            "25",
        );
    }


    #[test]
    fn init_edge_full() {
        test_ignore_whitespaces(
            "12
            .....
            .....
            ..S..
            .....
            .....",
            "169",
        );
        test_ignore_whitespaces(
            "12
            .....
            .#.#.
            ..S..
            .#.#.
            .....",
            "133",
        );
        test_ignore_whitespaces(
            "12
            .....
            ...#.
            ..S..
            .#.#.
            .....",
            "142",
        );
    }

    #[test]
    fn init_edge_2full() {
        test_ignore_whitespaces(
            "17
            .....
            .....
            ..S..
            .....
            .....",
            "324",
        );
    }

    #[test]
    fn random() {
        test_ignore_whitespaces(
            "10
            .......
            .......
            .......
            ...S...
            .......
            .......
            .......",
            "121",
        );
    }

    #[test]
    fn found() {
        test_ignore_whitespaces(
            "17
            .......
            .#.....
            .#.....
            ...S...
            ..#.##.
            ..#..#.
            .......",
            "277",
        );
    }
}
