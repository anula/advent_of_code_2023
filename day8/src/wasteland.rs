//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<String, Node>,
}

impl Node {
    fn from_string(line: &str) -> Node {
        lazy_static! {
            static ref NODE_RE : Regex = Regex::new(
                r"(?P<name>...) = \((?P<left>...), (?P<right>...)\)"
            ).unwrap();
        }

        let caps = NODE_RE.captures(line).unwrap();

        Node {
            name: caps.name("name").unwrap().as_str().to_string(),
            left: caps.name("left").unwrap().as_str().to_string(),
            right: caps.name("right").unwrap().as_str().to_string(),

        }
    }
}

impl Tree {
    fn from_lines<I>(lines: I) -> Tree
        where I: Iterator<Item = String>
    {
        let mut nodes = HashMap::new();

        for line in lines {
            let node = Node::from_string(&line);
            nodes.insert((&node.name).to_string(), node);
        }

        Tree {
            nodes: nodes,
        }
    }

    fn traverse_with_directions(&self, start: &str, dest: &str, dirs: &[Direction]) -> usize {
        let mut curr = start;
        let mut steps = 0;

        while curr != dest {
            let idx = steps % dirs.len();
            curr = match dirs[idx] {
                Direction::Left => &self.nodes[curr].left,
                Direction::Right => &self.nodes[curr].right,
            };
            steps += 1;
        }

        return steps;
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_string(line: &str) -> Vec<Direction> {
        let mut dirs = vec![];

        for c in line.chars() {
            if c == 'L' {
                dirs.push(Direction::Left)
            }
            if c == 'R' {
                dirs.push(Direction::Right)
            }
        }

        dirs
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut lines = BufReader::new(input).lines();

    let dirs = Direction::from_string(&lines.next().unwrap().unwrap());
    dprintln!("dirs: {:?}", dirs);

    // empty line
    let _ = lines.next();

    let tree = Tree::from_lines(lines.map(|l| l.unwrap()));
    dprintln!("tree: {:?}", tree);

    writeln!(output, "{}", tree.traverse_with_directions("AAA", "ZZZ", &dirs)).unwrap();
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
            "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)",
            "2",
        );
    }

    #[test]
    fn another() {
        test_ignore_whitespaces(
            "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)",
            "6",
        );
    }
}
